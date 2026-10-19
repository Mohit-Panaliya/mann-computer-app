use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InwardSlip {
    pub id: String,
    pub slip_no: String,
    pub customer_id: String,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub date: String,
    pub device_type: String,
    pub problem_description: Option<String>,
    pub parts_checklist: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub id: String,
    pub quotation_no: String,
    pub inward_id: Option<String>,
    pub customer_id: String,
    pub customer_name: Option<String>,
    pub items: Vec<QuotationItem>,
    pub total_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotationItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repair {
    pub id: String,
    pub inward_id: String,
    pub quotation_id: Option<String>,
    pub status: String,
    pub diagnosis: Option<String>,
    pub parts_used: Option<String>,
    pub labor_cost: f64,
    pub total_cost: f64,
    pub technician: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bill {
    pub id: String,
    pub bill_no: String,
    pub inward_id: Option<String>,
    pub repair_id: Option<String>,
    pub customer_id: String,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub items: Vec<BillItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total_amount: f64,
    pub payment_status: String,
    pub payment_method: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub purchase_price: f64,
    pub selling_price: f64,
    pub stock_quantity: i32,
    pub compatible_with: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartSale {
    pub id: String,
    pub part_id: String,
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub sale_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CctvQuote {
    pub id: String,
    pub quote_no: String,
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub items: Vec<QuotationItem>,
    pub total_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaptopQuote {
    pub id: String,
    pub quote_no: String,
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub specs: String,
    pub price: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRecord {
    pub id: String,
    pub inward_id: String,
    pub bill_id: Option<String>,
    pub customer_id: String,
    pub delivered_at: String,
    pub delivered_by: Option<String>,
    pub received_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_inward: i64,
    pub pending_repair: i64,
    pub ready_for_delivery: i64,
    pub delivered: i64,
    pub total_bills: i64,
    pub pending_bills: i64,
    pub today_revenue: f64,
    pub parts_in_stock: i64,
}
