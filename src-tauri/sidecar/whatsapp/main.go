package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"go.mau.fi/whatsmeow"
	"go.mau.fi/whatsmeow/appstate"
	waProto "go.mau.fi/whatsmeow/binary/proto"
	"go.mau.fi/whatsmeow/store"
	"go.mau.fi/whatsmeow/store/sqlstore"
	"go.mau.fi/whatsmeow/types"
	"go.mau.fi/whatsmeow/types/events"
	waLog "go.mau.fi/whatsmeow/util/log"
)

type Request struct {
	ID     string `json:"id"`
	Action string `json:"action"`
	Phone  string `json:"phone,omitempty"`
	Message string `json:"message,omitempty"`
	Caption string `json:"caption,omitempty"`
	Path   string `json:"path,omitempty"`
}

type Response struct {
	ID      string `json:"id"`
	Status  string `json:"status"`
	Error   string `json:"error,omitempty"`
	QRCode  string `json:"qr_code,omitempty"`
	Connected bool `json:"connected,omitempty"`
}

type Sidecar struct {
	client *whatsmeow.Client
	container *sqlstore.Container
	mu sync.Mutex
	qrChan chan string
	connected bool
}

func NewSidecar() (*Sidecar, error) {
	homeDir, _ := os.UserHomeDir()
	dbPath := filepath.Join(homeDir, ".mann_computer", "whatsapp.db")
	os.MkdirAll(filepath.Dir(dbPath), 0755)

	dbLog := waLog.Stdout("Database", "DEBUG", true)
	container, err := sqlstore.New("sqlite3", "file:"+dbPath+"?_foreign_keys=on", dbLog)
	if err != nil {
		return nil, fmt.Errorf("failed to create store: %w", err)
	}

	deviceStore, err := container.GetFirstDevice()
	if err != nil {
		return nil, fmt.Errorf("failed to get device: %w", err)
	}

	clientLog := waLog.Stdout("Client", "DEBUG", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)

	s := &Sidecar{
		client: client,
		container: container,
		qrChan: make(chan string, 1),
	}

	client.AddEventHandler(s.handleEvent)

	return s, nil
}

func (s *Sidecar) handleEvent(evt interface{}) {
	switch v := evt.(type) {
	case *events.AppStateSyncComplete:
		if v.Name == appstate.WAPatchCriticalBlock {
			s.connected = true
		}
	case *events.QR:
		select {
		case s.qrChan <- v.Codes[0]:
		default:
		}
	case *events.Connected:
		s.connected = true
	case *events.Disconnected:
		s.connected = false
	case *events.LoggedOut:
		s.connected = false
		s.container.Delete()
	}
}

func (s *Sidecar) Connect() (string, error) {
	if s.client.Store.ID == nil {
		qrChan := make(chan string)
		go func() {
			qr := <-s.qrChan
			qrChan <- qr
		}()

		err := s.client.Connect()
		if err != nil {
			return "", err
		}

		ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
		defer cancel()

		for {
			select {
			case qr := <-qrChan:
				return qr, nil
			case <-ctx.Done():
				return "", fmt.Errorf("QR timeout")
			}
		}
	}

	err := s.client.Connect()
	if err != nil {
		return "", err
	}
	return "", nil
}

func (s *Sidecar) IsConnected() bool {
	return s.client.IsConnected()
}

func (s *Sidecar) SendText(phone, message string) error {
	if !s.client.IsConnected() {
		return fmt.Errorf("not connected to WhatsApp")
	}

	jid, err := types.ParseJID(phone)
	if err != nil {
		return fmt.Errorf("invalid phone number: %w", err)
	}

	_, err = s.client.SendMessage(context.Background(), jid, &waProto.Message{
		Conversation: &message,
	})
	return err
}

func (s *Sidecar) SendDocument(phone, path, caption string) error {
	if !s.client.IsConnected() {
		return fmt.Errorf("not connected to WhatsApp")
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("failed to read file: %w", err)
	}

	jid, err := types.ParseJID(phone)
	if err != nil {
		return fmt.Errorf("invalid phone number: %w", err)
	}

	uploaded, err := s.client.Upload(context.Background(), data, whatsmeow.MediaDocument)
	if err != nil {
		return fmt.Errorf("failed to upload: %w", err)
	}

	_, err = s.client.SendMessage(context.Background(), jid, &waProto.Message{
		DocumentMessage: &waProto.DocumentMessage{
			URL:       &uploaded.URL,
			DirectPath: &uploaded.DirectPath,
			MediaKey:  uploaded.MediaKey,
			Mimetype:  &uploaded.Mimetype,
			FileName:  &filepath.Base(path),
			Caption:   &caption,
		},
	})
	return err
}

func (s *Sidecar) Disconnect() {
	s.client.Disconnect()
}

func normalizePhone(phone string) string {
	cleaned := strings.TrimSpace(phone)
	cleaned = strings.ReplaceAll(cleaned, " ", "")
	cleaned = strings.ReplaceAll(cleaned, "-", "")
	cleaned = strings.ReplaceAll(cleaned, "(", "")
	cleaned = strings.ReplaceAll(cleaned, ")", "")
	if !strings.HasPrefix(cleaned, "+") {
		if !strings.HasPrefix(cleaned, "91") {
			cleaned = "91" + cleaned
		}
		cleaned = "+" + cleaned
	}
	return cleaned
}

func main() {
	sidecar, err := NewSidecar()
	if err != nil {
		log.Fatalf("Failed to create sidecar: %v", err)
	}

	scanner := bufio.NewScanner(os.Stdin)
	encoder := json.NewEncoder(os.Stdout)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		var req Request
		if err := json.Unmarshal([]byte(line), &req); err != nil {
			encoder.Encode(Response{ID: "", Status: "error", Error: err.Error()})
			continue
		}

		var resp Response
		resp.ID = req.ID

		switch req.Action {
		case "connect":
			qr, err := sidecar.Connect()
			if err != nil {
				resp.Status = "error"
				resp.Error = err.Error()
			} else if qr != "" {
				resp.Status = "qr"
				resp.QRCode = qr
			} else {
				resp.Status = "connected"
				resp.Connected = true
			}

		case "status":
			resp.Status = "ok"
			resp.Connected = sidecar.IsConnected()

		case "send_text":
			phone := normalizePhone(req.Phone)
			err := sidecar.SendText(phone, req.Message)
			if err != nil {
				resp.Status = "error"
				resp.Error = err.Error()
			} else {
				resp.Status = "ok"
			}

		case "send_document":
			phone := normalizePhone(req.Phone)
			err := sidecar.SendDocument(phone, req.Path, req.Caption)
			if err != nil {
				resp.Status = "error"
				resp.Error = err.Error()
			} else {
				resp.Status = "ok"
			}

		case "disconnect":
			sidecar.Disconnect()
			resp.Status = "ok"

		default:
			resp.Status = "error"
			resp.Error = "unknown action: " + req.Action
		}

		encoder.Encode(resp)
		os.Stdout.Sync()
	}
}
