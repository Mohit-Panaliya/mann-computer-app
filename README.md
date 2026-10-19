# Mann Computer - Business Management App

A cross-platform desktop app for computer repair shops to manage inward slips, quotations, repairs, billing, parts inventory, CCTV quotes, laptop quotes, and delivery — with WhatsApp integration.

Built with **Tauri v2** (Rust backend + Svelte frontend) for minimal footprint on Windows, macOS, and Linux.

## Features

- **Dashboard** — Live stats: total inward, repairs in progress, ready for delivery, today's revenue, pending bills
- **Inward Slips** — Create slips matching your physical format with customer details, device type, problem description, parts checklist
- **Quotations** — Generate itemized quotations linked to inward slips or standalone
- **Repair Tracking** — Track status: pending → in_progress → completed → ready_for_delivery → delivered
- **Billing** — Generate bills with automatic tax calculation, payment tracking, PDF export
- **Delivery** — Mark items as delivered with receiver details
- **Parts Inventory** — Track stock, record sales, low-stock alerts
- **CCTV Quotes** — Quick-add common CCTV items (Hikvision, Dahua, DVRs, cables)
- **Laptop Quotes** — Quote laptops with specs and pricing
- **WhatsApp Integration** — Send bills, quotations, and status updates via WhatsApp (Go sidecar using whatsmeow)
- **PDF Generation** — Generate inward slips, bills, and quotations as PDFs (pure Rust, no browser needed)
- **SQLite Database** — Local, single-file database. No server required

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5 + TypeScript + Vite |
| Backend | Rust (Tauri v2) |
| Database | SQLite (rusqlite) |
| PDF | printpdf (pure Rust) |
| WhatsApp | Go sidecar (whatsmeow) |
| Build | Cargo + npm |

## Project Structure

```
mann-computer-app/
├── src/                      # Svelte frontend
│   ├── pages/                # Dashboard, InwardSlips, Quotations, etc.
│   ├── lib/                  # API client, utilities
│   └── App.svelte            # Main app with sidebar navigation
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs           # Tauri entry point
│   │   ├── db.rs             # SQLite setup & migrations
│   │   ├── commands.rs       # All Tauri commands (API layer)
│   │   ├── models.rs         # Data models
│   │   └── pdf.rs            # PDF generation
│   └── sidecar/
│       └── whatsapp/         # Go WhatsApp sidecar
└── package.json
```

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (latest stable)
- [Go](https://go.dev/) (v1.24+, for WhatsApp sidecar)

### Install & Run

```bash
# Clone the repository
git clone https://github.com/Mohit-Panaliya/mann-computer-app.git
cd mann-computer-app

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build for Production

```bash
# Build for current platform
npm run tauri build

# Output in: src-tauri/target/release/bundle/
```

## WhatsApp Integration

The app includes a Go sidecar that uses the [whatsmeow](https://github.com/tulir/whatsmeow) library to connect to WhatsApp Web without a browser.

### Setup

1. Build the sidecar:
   ```bash
   cd src-tauri/sidecar/whatsapp
   go build -o ../../whatsapp-sidecar
   cd ../..
   ```

2. On first run, the sidecar generates a QR code
3. Scan with your WhatsApp app (Settings → Linked Devices)
4. Session is saved locally — no need to scan again

### Features

- Send bills as PDF documents
- Send quotation details as text
- Send repair status updates
- Send delivery notifications

## Database

SQLite database is created automatically at:
- **Windows:** `%APPDATA%/com.manncomputer.app/mann_computer.db`
- **macOS:** `~/Library/Application Support/com.manncomputer.app/mann_computer.db`
- **Linux:** `~/.local/share/com.manncomputer.app/mann_computer.db`

### Backup

Copy the `.db` file to backup. No server or cloud sync needed.

## Shop Configuration

Edit shop details in the database `settings` table:
- Shop name, address, phone, email
- Tax rate (default: 18%)
- Currency (default: INR)
- Auto-increment counters for slip/quote/bill numbers

## License

Private — Mann Computer, Vallabh Vidhyanagar, Anand, Gujarat
