use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Local;

pub struct Db {
    pub conn: Mutex<Connection>,
}

impl Db {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)?;
        let db = Db { conn: Mutex::new(conn) };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS customers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS inward_slips (
                id TEXT PRIMARY KEY,
                slip_no TEXT UNIQUE NOT NULL,
                customer_id TEXT NOT NULL,
                date TEXT NOT NULL,
                device_type TEXT NOT NULL,
                problem_description TEXT,
                parts_checklist TEXT,
                remarks TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS quotations (
                id TEXT PRIMARY KEY,
                quotation_no TEXT UNIQUE NOT NULL,
                inward_id TEXT,
                customer_id TEXT NOT NULL,
                items TEXT NOT NULL,
                total_amount REAL NOT NULL,
                status TEXT NOT NULL DEFAULT 'draft',
                notes TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (inward_id) REFERENCES inward_slips(id),
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS repairs (
                id TEXT PRIMARY KEY,
                inward_id TEXT NOT NULL,
                quotation_id TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                diagnosis TEXT,
                parts_used TEXT,
                labor_cost REAL DEFAULT 0,
                total_cost REAL DEFAULT 0,
                technician TEXT,
                start_date TEXT,
                end_date TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (inward_id) REFERENCES inward_slips(id),
                FOREIGN KEY (quotation_id) REFERENCES quotations(id)
            );
            CREATE TABLE IF NOT EXISTS bills (
                id TEXT PRIMARY KEY,
                bill_no TEXT UNIQUE NOT NULL,
                inward_id TEXT,
                repair_id TEXT,
                customer_id TEXT NOT NULL,
                items TEXT NOT NULL,
                subtotal REAL NOT NULL,
                tax REAL DEFAULT 0,
                total_amount REAL NOT NULL,
                payment_status TEXT NOT NULL DEFAULT 'pending',
                payment_method TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (inward_id) REFERENCES inward_slips(id),
                FOREIGN KEY (repair_id) REFERENCES repairs(id),
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS parts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT,
                description TEXT,
                purchase_price REAL DEFAULT 0,
                selling_price REAL DEFAULT 0,
                stock_quantity INTEGER DEFAULT 0,
                compatible_with TEXT,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS part_sales (
                id TEXT PRIMARY KEY,
                part_id TEXT NOT NULL,
                customer_id TEXT,
                customer_name TEXT,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                sale_date TEXT NOT NULL,
                FOREIGN KEY (part_id) REFERENCES parts(id)
            );
            CREATE TABLE IF NOT EXISTS cctv_quotes (
                id TEXT PRIMARY KEY,
                quote_no TEXT UNIQUE NOT NULL,
                customer_id TEXT NOT NULL,
                customer_name TEXT,
                customer_phone TEXT,
                items TEXT NOT NULL,
                total_amount REAL NOT NULL,
                status TEXT NOT NULL DEFAULT 'draft',
                notes TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS laptop_quotes (
                id TEXT PRIMARY KEY,
                quote_no TEXT UNIQUE NOT NULL,
                customer_id TEXT NOT NULL,
                customer_name TEXT,
                customer_phone TEXT,
                specs TEXT NOT NULL,
                price REAL NOT NULL,
                status TEXT NOT NULL DEFAULT 'draft',
                notes TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS delivery_records (
                id TEXT PRIMARY KEY,
                inward_id TEXT NOT NULL,
                bill_id TEXT,
                customer_id TEXT NOT NULL,
                delivered_at TEXT NOT NULL,
                delivered_by TEXT,
                received_by TEXT,
                notes TEXT,
                FOREIGN KEY (inward_id) REFERENCES inward_slips(id),
                FOREIGN KEY (bill_id) REFERENCES bills(id),
                FOREIGN KEY (customer_id) REFERENCES customers(id)
            );
            CREATE TABLE IF NOT EXISTS whatsapp_messages (
                id TEXT PRIMARY KEY,
                phone TEXT NOT NULL,
                message_type TEXT NOT NULL,
                content TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                sent_at TEXT,
                error TEXT
            );
            INSERT OR IGNORE INTO settings (key, value) VALUES
                ('shop_name', 'Mann Computer'),
                ('shop_address', 'G14, Panch Ratna Complex, Opp. Honest Restaurant, Mota Bazar, Vallabh Vidhyanagar - 388120, Ta & Dist. Anand'),
                ('shop_phone', '9824638016'),
                ('shop_email', 'manncomputervvn@gmail.com'),
                ('jurisdiction', 'Subject to Anand Jurisdiction.'),
                ('inward_counter', '0'),
                ('quotation_counter', '0'),
                ('bill_counter', '0'),
                ('cctv_quote_counter', '0'),
                ('laptop_quote_counter', '0'),
                ('tax_rate', '18'),
                ('currency', 'INR');
            "#
        )?;
        Ok(())
    }

    pub fn next_slip_no(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'inward_counter'")?;
        let counter: i64 = stmt.query_row([], |row| row.get(0))?;
        conn.execute("UPDATE settings SET value = ?1 WHERE key = 'inward_counter'", params![(counter + 1).to_string()])?;
        Ok(format!("{:04}", counter + 1))
    }

    pub fn next_quotation_no(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'quotation_counter'")?;
        let counter: i64 = stmt.query_row([], |row| row.get(0))?;
        conn.execute("UPDATE settings SET value = ?1 WHERE key = 'quotation_counter'", params![(counter + 1).to_string()])?;
        Ok(format!("Q{:04}", counter + 1))
    }

    pub fn next_bill_no(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'bill_counter'")?;
        let counter: i64 = stmt.query_row([], |row| row.get(0))?;
        conn.execute("UPDATE settings SET value = ?1 WHERE key = 'bill_counter'", params![(counter + 1).to_string()])?;
        Ok(format!("B{:04}", counter + 1))
    }

    pub fn next_cctv_quote_no(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'cctv_quote_counter'")?;
        let counter: i64 = stmt.query_row([], |row| row.get(0))?;
        conn.execute("UPDATE settings SET value = ?1 WHERE key = 'cctv_quote_counter'", params![(counter + 1).to_string()])?;
        Ok(format!("CCTV{:04}", counter + 1))
    }

    pub fn next_laptop_quote_no(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'laptop_quote_counter'")?;
        let counter: i64 = stmt.query_row([], |row| row.get(0))?;
        conn.execute("UPDATE settings SET value = ?1 WHERE key = 'laptop_quote_counter'", params![(counter + 1).to_string()])?;
        Ok(format!("LT{:04}", counter + 1))
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)", params![key, value])?;
        Ok(())
    }
}
