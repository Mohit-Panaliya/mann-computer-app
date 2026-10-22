use tauri::State;
use rusqlite::params;
use uuid::Uuid;
use chrono::Local;
use std::path::PathBuf;
use std::sync::Arc;

use crate::db::Db;
use crate::models::*;
use crate::pdf;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Db>,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        let db = Arc::new(Db::new(db_path).map_err(|e| format!("Database init failed: {}", e))?);
        Ok(AppState { db })
    }
}

#[tauri::command]
pub fn health_check() -> Result<String, String> {
    Ok("ok".to_string())
}

#[tauri::command]
pub fn get_customers(state: State<AppState>) -> Result<Vec<Customer>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, phone, address, created_at FROM customers ORDER BY created_at DESC").map_err(|e| e.to_string())?;
    let customers = stmt.query_map([], |row| {
        Ok(Customer { id: row.get(0)?, name: row.get(1)?, phone: row.get(2)?, address: row.get(3)?, created_at: row.get(4)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for c in customers { result.push(c.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_customer(state: State<AppState>, name: String, phone: String, address: Option<String>) -> Result<Customer, String> {
    if name.trim().is_empty() {
        return Err("Customer name is required".to_string());
    }
    if phone.trim().is_empty() {
        return Err("Customer phone is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO customers (id, name, phone, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)", params![id, name, phone, address, now]).map_err(|e| format!("Failed to insert customer: {}", e))?;
    Ok(Customer { id, name, phone, address, created_at: now })
}

#[tauri::command]
pub fn update_customer(state: State<AppState>, id: String, name: String, phone: String, address: Option<String>) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Customer name is required".to_string());
    }
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("UPDATE customers SET name=?1, phone=?2, address=?3 WHERE id=?4", params![name, phone, address, id]).map_err(|e| format!("Failed to update customer: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_inward_slips(state: State<AppState>) -> Result<Vec<InwardSlip>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT s.id, s.slip_no, s.customer_id, c.name, c.phone, s.date, s.device_type, s.problem_description, s.parts_checklist, s.remarks, s.status, s.created_at FROM inward_slips s LEFT JOIN customers c ON s.customer_id = c.id ORDER BY s.created_at DESC").map_err(|e| e.to_string())?;
    let slips = stmt.query_map([], |row| {
        Ok(InwardSlip { id: row.get(0)?, slip_no: row.get(1)?, customer_id: row.get(2)?, customer_name: row.get(3)?, customer_phone: row.get(4)?, date: row.get(5)?, device_type: row.get(6)?, problem_description: row.get(7)?, parts_checklist: row.get(8)?, remarks: row.get(9)?, status: row.get(10)?, created_at: row.get(11)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for s in slips { result.push(s.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn get_inward_slip(state: State<AppState>, id: String) -> Result<Option<InwardSlip>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT s.id, s.slip_no, s.customer_id, c.name, c.phone, s.date, s.device_type, s.problem_description, s.parts_checklist, s.remarks, s.status, s.created_at FROM inward_slips s LEFT JOIN customers c ON s.customer_id = c.id WHERE s.id = ?1").map_err(|e| e.to_string())?;
    let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Some(InwardSlip {
            id: row.get(0).map_err(|e| e.to_string())?,
            slip_no: row.get(1).map_err(|e| e.to_string())?,
            customer_id: row.get(2).map_err(|e| e.to_string())?,
            customer_name: row.get(3).map_err(|e| e.to_string())?,
            customer_phone: row.get(4).map_err(|e| e.to_string())?,
            date: row.get(5).map_err(|e| e.to_string())?,
            device_type: row.get(6).map_err(|e| e.to_string())?,
            problem_description: row.get(7).map_err(|e| e.to_string())?,
            parts_checklist: row.get(8).map_err(|e| e.to_string())?,
            remarks: row.get(9).map_err(|e| e.to_string())?,
            status: row.get(10).map_err(|e| e.to_string())?,
            created_at: row.get(11).map_err(|e| e.to_string())?,
        }))
    } else { Ok(None) }
}

#[tauri::command]
pub fn create_inward_slip(state: State<AppState>, customer_id: String, device_type: String, problem_description: Option<String>, parts_checklist: Option<String>, remarks: Option<String>) -> Result<InwardSlip, String> {
    if customer_id.trim().is_empty() {
        return Err("Customer is required".to_string());
    }
    if device_type.trim().is_empty() {
        return Err("Device type is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let slip_no = state.db.next_slip_no().map_err(|e| format!("Failed to generate slip number: {}", e))?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let date = Local::now().format("%Y-%m-%d").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO inward_slips (id, slip_no, customer_id, date, device_type, problem_description, parts_checklist, remarks, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'pending', ?9)", params![id, slip_no, customer_id, date, device_type, problem_description, parts_checklist, remarks, now]).map_err(|e| format!("Failed to insert inward slip: {}", e))?;
    Ok(InwardSlip { id, slip_no, customer_id, customer_name: None, customer_phone: None, date, device_type, problem_description, parts_checklist, remarks, status: "pending".to_string(), created_at: now })
}

#[tauri::command]
pub fn update_inward_slip_status(state: State<AppState>, id: String, status: String) -> Result<(), String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("UPDATE inward_slips SET status=?1 WHERE id=?2", params![status, id]).map_err(|e| format!("Failed to update status: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_quotations(state: State<AppState>) -> Result<Vec<Quotation>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT q.id, q.quotation_no, q.inward_id, q.customer_id, c.name, q.items, q.total_amount, q.status, q.notes, q.created_at FROM quotations q LEFT JOIN customers c ON q.customer_id = c.id ORDER BY q.created_at DESC").map_err(|e| e.to_string())?;
    let quotes = stmt.query_map([], |row| {
        let items_json: String = row.get(5)?;
        let items: Vec<QuotationItem> = serde_json::from_str(&items_json).unwrap_or_default();
        Ok(Quotation { id: row.get(0)?, quotation_no: row.get(1)?, inward_id: row.get(2)?, customer_id: row.get(3)?, customer_name: row.get(4)?, items, total_amount: row.get(6)?, status: row.get(7)?, notes: row.get(8)?, created_at: row.get(9)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for q in quotes { result.push(q.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_quotation(state: State<AppState>, inward_id: Option<String>, customer_id: String, items: Vec<QuotationItem>, notes: Option<String>) -> Result<Quotation, String> {
    if customer_id.trim().is_empty() {
        return Err("Customer is required".to_string());
    }
    if items.is_empty() {
        return Err("At least one item is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let quotation_no = state.db.next_quotation_no().map_err(|e| format!("Failed to generate quotation number: {}", e))?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let total: f64 = items.iter().map(|i| i.total).sum();
    let items_json = serde_json::to_string(&items).map_err(|e| format!("Failed to serialize items: {}", e))?;
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO quotations (id, quotation_no, inward_id, customer_id, items, total_amount, status, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'draft', ?7, ?8)", params![id, quotation_no, inward_id, customer_id, items_json, total, notes, now]).map_err(|e| format!("Failed to insert quotation: {}", e))?;
    Ok(Quotation { id, quotation_no, inward_id, customer_id, customer_name: None, items, total_amount: total, status: "draft".to_string(), notes, created_at: now })
}

#[tauri::command]
pub fn get_repairs(state: State<AppState>) -> Result<Vec<Repair>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, inward_id, quotation_id, status, diagnosis, parts_used, labor_cost, total_cost, technician, start_date, end_date, created_at FROM repairs ORDER BY created_at DESC").map_err(|e| e.to_string())?;
    let repairs = stmt.query_map([], |row| {
        Ok(Repair { id: row.get(0)?, inward_id: row.get(1)?, quotation_id: row.get(2)?, status: row.get(3)?, diagnosis: row.get(4)?, parts_used: row.get(5)?, labor_cost: row.get(6)?, total_cost: row.get(7)?, technician: row.get(8)?, start_date: row.get(9)?, end_date: row.get(10)?, created_at: row.get(11)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for r in repairs { result.push(r.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_repair(state: State<AppState>, inward_id: String, quotation_id: Option<String>, diagnosis: Option<String>, labor_cost: f64, technician: Option<String>) -> Result<Repair, String> {
    if inward_id.trim().is_empty() {
        return Err("Inward slip is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let today = Local::now().format("%Y-%m-%d").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO repairs (id, inward_id, quotation_id, status, diagnosis, parts_used, labor_cost, total_cost, technician, start_date, created_at) VALUES (?1, ?2, ?3, 'in_progress', ?4, '[]', ?5, ?5, ?6, ?7, ?8)", params![id, inward_id, quotation_id, diagnosis, labor_cost, technician, today, now]).map_err(|e| format!("Failed to insert repair: {}", e))?;
    conn.execute("UPDATE inward_slips SET status='repairing' WHERE id=?1", params![inward_id]).map_err(|e| format!("Failed to update inward slip status: {}", e))?;
    Ok(Repair { id, inward_id, quotation_id, status: "in_progress".to_string(), diagnosis, parts_used: Some("[]".to_string()), labor_cost, total_cost: labor_cost, technician, start_date: Some(today), end_date: None, created_at: now })
}

#[tauri::command]
pub fn update_repair(state: State<AppState>, id: String, status: String, diagnosis: Option<String>, parts_used: Option<String>, labor_cost: f64, total_cost: f64, end_date: Option<String>) -> Result<(), String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("UPDATE repairs SET status=?1, diagnosis=?2, parts_used=?3, labor_cost=?4, total_cost=?5, end_date=?6 WHERE id=?7", params![status, diagnosis, parts_used, labor_cost, total_cost, end_date, id]).map_err(|e| format!("Failed to update repair: {}", e))?;
    if status == "completed" || status == "ready_for_delivery" {
        let inward_id: String = conn.query_row("SELECT inward_id FROM repairs WHERE id=?1", params![id], |r| r.get(0)).map_err(|e| format!("Failed to get inward_id: {}", e))?;
        let new_status = if status == "ready_for_delivery" { "ready" } else { "completed" };
        conn.execute("UPDATE inward_slips SET status=?1 WHERE id=?2", params![new_status, inward_id]).map_err(|e| format!("Failed to update inward slip status: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_bills(state: State<AppState>) -> Result<Vec<Bill>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT b.id, b.bill_no, b.inward_id, b.repair_id, b.customer_id, c.name, c.phone, b.items, b.subtotal, b.tax, b.total_amount, b.payment_status, b.payment_method, b.created_at FROM bills b LEFT JOIN customers c ON b.customer_id = c.id ORDER BY b.created_at DESC").map_err(|e| e.to_string())?;
    let bills = stmt.query_map([], |row| {
        let items_json: String = row.get(8)?;
        let items: Vec<BillItem> = serde_json::from_str(&items_json).unwrap_or_default();
        Ok(Bill { id: row.get(0)?, bill_no: row.get(1)?, inward_id: row.get(2)?, repair_id: row.get(3)?, customer_id: row.get(4)?, customer_name: row.get(5)?, customer_phone: row.get(6)?, items, subtotal: row.get(9)?, tax: row.get(10)?, total_amount: row.get(11)?, payment_status: row.get(12)?, payment_method: row.get(13)?, created_at: row.get(14)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for b in bills { result.push(b.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_bill(state: State<AppState>, inward_id: Option<String>, repair_id: Option<String>, customer_id: String, items: Vec<BillItem>, payment_method: Option<String>) -> Result<Bill, String> {
    if customer_id.trim().is_empty() {
        return Err("Customer is required".to_string());
    }
    if items.is_empty() {
        return Err("At least one item is required".to_string());
    }
    let payment_method = payment_method.unwrap_or_else(|| "Cash".to_string());
    let id = Uuid::new_v4().to_string();
    let bill_no = state.db.next_bill_no().map_err(|e| format!("Failed to generate bill number: {}", e))?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let subtotal: f64 = items.iter().map(|i| i.total).sum();
    let tax_rate: f64 = state.db.get_setting("tax_rate").ok().flatten().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let tax = subtotal * tax_rate / 100.0;
    let total = subtotal + tax;
    let items_json = serde_json::to_string(&items).map_err(|e| format!("Failed to serialize items: {}", e))?;
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO bills (id, bill_no, inward_id, repair_id, customer_id, items, subtotal, tax, total_amount, payment_status, payment_method, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'pending', ?10, ?11)", params![id, bill_no, inward_id, repair_id, customer_id, items_json, subtotal, tax, total, payment_method, now]).map_err(|e| format!("Failed to insert bill: {}", e))?;
    let mut stmt = conn.prepare("SELECT name, phone FROM customers WHERE id=?1").map_err(|e| e.to_string())?;
    let mut rows = stmt.query(params![customer_id]).map_err(|e| e.to_string())?;
    let (cname, cphone): (Option<String>, Option<String>) = match rows.next().map_err(|e| e.to_string())? {
        Some(row) => (row.get(0).ok(), row.get(1).ok()),
        None => (None, None),
    };
    Ok(Bill { id, bill_no, inward_id, repair_id, customer_id, customer_name: cname, customer_phone: cphone, items, subtotal, tax, total_amount: total, payment_status: "pending".to_string(), payment_method: Some(payment_method), created_at: now })
}

#[tauri::command]
pub fn update_bill_payment(state: State<AppState>, id: String, payment_status: String, payment_method: Option<String>) -> Result<(), String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("UPDATE bills SET payment_status=?1, payment_method=?2 WHERE id=?3", params![payment_status, payment_method, id]).map_err(|e| format!("Failed to update payment: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_parts(state: State<AppState>) -> Result<Vec<Part>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, category, description, purchase_price, selling_price, stock_quantity, compatible_with, created_at FROM parts ORDER BY name").map_err(|e| e.to_string())?;
    let parts = stmt.query_map([], |row| {
        Ok(Part { id: row.get(0)?, name: row.get(1)?, category: row.get(2)?, description: row.get(3)?, purchase_price: row.get(4)?, selling_price: row.get(5)?, stock_quantity: row.get(6)?, compatible_with: row.get(7)?, created_at: row.get(8)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for p in parts { result.push(p.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_part(state: State<AppState>, name: String, category: Option<String>, description: Option<String>, purchase_price: f64, selling_price: f64, stock_quantity: i32, compatible_with: Option<String>) -> Result<Part, String> {
    if name.trim().is_empty() {
        return Err("Part name is required".to_string());
    }
    if selling_price <= 0.0 {
        return Err("Selling price must be greater than 0".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO parts (id, name, category, description, purchase_price, selling_price, stock_quantity, compatible_with, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", params![id, name, category, description, purchase_price, selling_price, stock_quantity, compatible_with, now]).map_err(|e| format!("Failed to insert part: {}", e))?;
    Ok(Part { id, name, category, description, purchase_price, selling_price, stock_quantity, compatible_with, created_at: now })
}

#[tauri::command]
pub fn update_part_stock(state: State<AppState>, id: String, quantity: i32) -> Result<(), String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("UPDATE parts SET stock_quantity = stock_quantity + ?1 WHERE id=?2", params![quantity, id]).map_err(|e| format!("Failed to update stock: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_part_sales(state: State<AppState>) -> Result<Vec<PartSale>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, part_id, customer_id, customer_name, quantity, unit_price, total_price, sale_date FROM part_sales ORDER BY sale_date DESC").map_err(|e| e.to_string())?;
    let sales = stmt.query_map([], |row| {
        Ok(PartSale { id: row.get(0)?, part_id: row.get(1)?, customer_id: row.get(2)?, customer_name: row.get(3)?, quantity: row.get(4)?, unit_price: row.get(5)?, total_price: row.get(6)?, sale_date: row.get(7)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for s in sales { result.push(s.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_part_sale(state: State<AppState>, part_id: String, customer_id: Option<String>, customer_name: Option<String>, quantity: i32, unit_price: f64) -> Result<PartSale, String> {
    if part_id.trim().is_empty() {
        return Err("Part is required".to_string());
    }
    if quantity <= 0 {
        return Err("Quantity must be greater than 0".to_string());
    }
    if unit_price <= 0.0 {
        return Err("Unit price must be greater than 0".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let total = quantity as f64 * unit_price;
    let today = Local::now().format("%Y-%m-%d").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO part_sales (id, part_id, customer_id, customer_name, quantity, unit_price, total_price, sale_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![id, part_id, customer_id, customer_name, quantity, unit_price, total, today]).map_err(|e| format!("Failed to insert part sale: {}", e))?;
    conn.execute("UPDATE parts SET stock_quantity = stock_quantity - ?1 WHERE id=?2", params![quantity, part_id]).map_err(|e| format!("Failed to update part stock: {}", e))?;
    Ok(PartSale { id, part_id, customer_id, customer_name, quantity, unit_price, total_price: total, sale_date: today })
}

#[tauri::command]
pub fn get_cctv_quotes(state: State<AppState>) -> Result<Vec<CctvQuote>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, quote_no, customer_id, customer_name, customer_phone, items, total_amount, status, notes, created_at FROM cctv_quotes ORDER BY created_at DESC").map_err(|e| e.to_string())?;
    let quotes = stmt.query_map([], |row| {
        let items_json: String = row.get(5)?;
        let items: Vec<QuotationItem> = serde_json::from_str(&items_json).unwrap_or_default();
        Ok(CctvQuote { id: row.get(0)?, quote_no: row.get(1)?, customer_id: row.get(2)?, customer_name: row.get(3)?, customer_phone: row.get(4)?, items, total_amount: row.get(6)?, status: row.get(7)?, notes: row.get(8)?, created_at: row.get(9)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for q in quotes { result.push(q.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_cctv_quote(state: State<AppState>, customer_name: String, customer_phone: String, items: Vec<QuotationItem>, notes: Option<String>) -> Result<CctvQuote, String> {
    if customer_name.trim().is_empty() {
        return Err("Customer name is required".to_string());
    }
    if customer_phone.trim().is_empty() {
        return Err("Customer phone is required".to_string());
    }
    if items.is_empty() {
        return Err("At least one item is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let quote_no = state.db.next_cctv_quote_no().map_err(|e| format!("Failed to generate quote number: {}", e))?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let total: f64 = items.iter().map(|i| i.total).sum();
    let items_json = serde_json::to_string(&items).map_err(|e| format!("Failed to serialize items: {}", e))?;
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO cctv_quotes (id, quote_no, customer_id, customer_name, customer_phone, items, total_amount, status, notes, created_at) VALUES (?1, ?2, NULL, ?3, ?4, ?5, ?6, 'draft', ?7, ?8)", params![id, quote_no, customer_name, customer_phone, items_json, total, notes, now]).map_err(|e| format!("Failed to insert CCTV quote: {}", e))?;
    Ok(CctvQuote { id, quote_no, customer_id: None, customer_name: Some(customer_name), customer_phone: Some(customer_phone), items, total_amount: total, status: "draft".to_string(), notes, created_at: now })
}

#[tauri::command]
pub fn get_laptop_quotes(state: State<AppState>) -> Result<Vec<LaptopQuote>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, quote_no, customer_id, customer_name, customer_phone, specs, price, status, notes, created_at FROM laptop_quotes ORDER BY created_at DESC").map_err(|e| e.to_string())?;
    let quotes = stmt.query_map([], |row| {
        Ok(LaptopQuote { id: row.get(0)?, quote_no: row.get(1)?, customer_id: row.get(2)?, customer_name: row.get(3)?, customer_phone: row.get(4)?, specs: row.get(5)?, price: row.get(6)?, status: row.get(7)?, notes: row.get(8)?, created_at: row.get(9)? })
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for q in quotes { result.push(q.map_err(|e| e.to_string())?); }
    Ok(result)
}

#[tauri::command]
pub fn create_laptop_quote(state: State<AppState>, customer_name: String, customer_phone: String, specs: String, price: f64, notes: Option<String>) -> Result<LaptopQuote, String> {
    if customer_name.trim().is_empty() {
        return Err("Customer name is required".to_string());
    }
    if customer_phone.trim().is_empty() {
        return Err("Customer phone is required".to_string());
    }
    if specs.trim().is_empty() {
        return Err("Specifications are required".to_string());
    }
    if price <= 0.0 {
        return Err("Price must be greater than 0".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let quote_no = state.db.next_laptop_quote_no().map_err(|e| format!("Failed to generate quote number: {}", e))?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO laptop_quotes (id, quote_no, customer_id, customer_name, customer_phone, specs, price, status, notes, created_at) VALUES (?1, ?2, NULL, ?3, ?4, ?5, ?6, 'draft', ?7, ?8)", params![id, quote_no, customer_name, customer_phone, specs, price, notes, now]).map_err(|e| format!("Failed to insert laptop quote: {}", e))?;
    Ok(LaptopQuote { id, quote_no, customer_id: None, customer_name: Some(customer_name), customer_phone: Some(customer_phone), specs, price, status: "draft".to_string(), notes, created_at: now })
}

#[tauri::command]
pub fn create_delivery(state: State<AppState>, inward_id: String, bill_id: Option<String>, customer_id: String, delivered_by: Option<String>, received_by: Option<String>, notes: Option<String>) -> Result<DeliveryRecord, String> {
    if inward_id.trim().is_empty() {
        return Err("Inward slip is required".to_string());
    }
    if customer_id.trim().is_empty() {
        return Err("Customer is required".to_string());
    }
    let id = Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO delivery_records (id, inward_id, bill_id, customer_id, delivered_at, delivered_by, received_by, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![id, inward_id, bill_id, customer_id, now, delivered_by, received_by, notes]).map_err(|e| format!("Failed to insert delivery: {}", e))?;
    conn.execute("UPDATE inward_slips SET status='delivered' WHERE id=?1", params![inward_id]).map_err(|e| format!("Failed to update inward slip status: {}", e))?;
    Ok(DeliveryRecord { id, inward_id, bill_id, customer_id, delivered_at: now, delivered_by, received_by, notes })
}

#[tauri::command]
pub fn get_dashboard_stats(state: State<AppState>) -> Result<DashboardStats, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let total_inward: i64 = conn.query_row("SELECT COUNT(*) FROM inward_slips", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let pending_repair: i64 = conn.query_row("SELECT COUNT(*) FROM inward_slips WHERE status='repairing'", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let ready: i64 = conn.query_row("SELECT COUNT(*) FROM inward_slips WHERE status='ready'", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let delivered: i64 = conn.query_row("SELECT COUNT(*) FROM inward_slips WHERE status='delivered'", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let total_bills: i64 = conn.query_row("SELECT COUNT(*) FROM bills", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let pending_bills: i64 = conn.query_row("SELECT COUNT(*) FROM bills WHERE payment_status='pending'", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let today = Local::now().format("%Y-%m-%d").to_string();
    let today_revenue: f64 = conn.query_row("SELECT COALESCE(SUM(total_amount), 0) FROM bills WHERE payment_status='paid' AND created_at LIKE ?1", params![format!("{}%", today)], |r| r.get(0)).map_err(|e| e.to_string())?;
    let parts_in_stock: i64 = conn.query_row("SELECT COALESCE(SUM(stock_quantity), 0) FROM parts", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    Ok(DashboardStats { total_inward, pending_repair, ready_for_delivery: ready, delivered, total_bills, pending_bills, today_revenue, parts_in_stock })
}

#[tauri::command]
pub fn generate_pdf(state: State<AppState>, doc_type: String, id: String) -> Result<String, String> {
    match doc_type.as_str() {
        "inward_slip" => {
            let slip = get_inward_slip(state.clone(), id)?.ok_or("Inward slip not found")?;
            pdf::generate_inward_slip_pdf(&slip, &state.db).map_err(|e| format!("PDF generation failed: {}", e))
        }
        "bill" => {
            let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
            let mut stmt = conn.prepare("SELECT b.id, b.bill_no, b.inward_id, b.repair_id, b.customer_id, c.name, c.phone, b.items, b.subtotal, b.tax, b.total_amount, b.payment_status, b.payment_method, b.created_at FROM bills b LEFT JOIN customers c ON b.customer_id = c.id WHERE b.id=?1").map_err(|e| e.to_string())?;
            let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
            let row = rows.next().map_err(|e| e.to_string())?.ok_or("Bill not found")?;
            let items_json: String = row.get(8).map_err(|e| e.to_string())?;
            let items: Vec<BillItem> = serde_json::from_str(&items_json).unwrap_or_default();
            let bill = Bill {
                id: row.get(0).map_err(|e| e.to_string())?,
                bill_no: row.get(1).map_err(|e| e.to_string())?,
                inward_id: row.get(2).map_err(|e| e.to_string())?,
                repair_id: row.get(3).map_err(|e| e.to_string())?,
                customer_id: row.get(4).map_err(|e| e.to_string())?,
                customer_name: row.get(5).map_err(|e| e.to_string())?,
                customer_phone: row.get(6).map_err(|e| e.to_string())?,
                items,
                subtotal: row.get(9).map_err(|e| e.to_string())?,
                tax: row.get(10).map_err(|e| e.to_string())?,
                total_amount: row.get(11).map_err(|e| e.to_string())?,
                payment_status: row.get(12).map_err(|e| e.to_string())?,
                payment_method: row.get(13).map_err(|e| e.to_string())?,
                created_at: row.get(14).map_err(|e| e.to_string())?,
            };
            pdf::generate_bill_pdf(&bill, &state.db).map_err(|e| format!("PDF generation failed: {}", e))
        }
        "quotation" => {
            let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
            let mut stmt = conn.prepare("SELECT q.id, q.quotation_no, q.inward_id, q.customer_id, c.name, q.items, q.total_amount, q.status, q.notes, q.created_at FROM quotations q LEFT JOIN customers c ON q.customer_id = c.id WHERE q.id=?1").map_err(|e| e.to_string())?;
            let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;
            let row = rows.next().map_err(|e| e.to_string())?.ok_or("Quotation not found")?;
            let items_json: String = row.get(5).map_err(|e| e.to_string())?;
            let items: Vec<QuotationItem> = serde_json::from_str(&items_json).unwrap_or_default();
            let quote = Quotation {
                id: row.get(0).map_err(|e| e.to_string())?,
                quotation_no: row.get(1).map_err(|e| e.to_string())?,
                inward_id: row.get(2).map_err(|e| e.to_string())?,
                customer_id: row.get(3).map_err(|e| e.to_string())?,
                customer_name: row.get(4).map_err(|e| e.to_string())?,
                items,
                total_amount: row.get(6).map_err(|e| e.to_string())?,
                status: row.get(7).map_err(|e| e.to_string())?,
                notes: row.get(8).map_err(|e| e.to_string())?,
                created_at: row.get(9).map_err(|e| e.to_string())?,
            };
            pdf::generate_quotation_pdf(&quote, &state.db).map_err(|e| format!("PDF generation failed: {}", e))
        }
        _ => Err("Unknown document type".to_string()),
    }
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<std::collections::HashMap<String, String>, String> {
    let conn = state.db.lock_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT key, value FROM settings").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))).map_err(|e| e.to_string())?;
    let mut map = std::collections::HashMap::new();
    for row in rows { let (k, v) = row.map_err(|e| e.to_string())?; map.insert(k, v); }
    Ok(map)
}

#[tauri::command]
pub fn update_setting(state: State<AppState>, key: String, value: String) -> Result<(), String> {
    state.db.set_setting(&key, &value).map_err(|e| e.to_string())
}
