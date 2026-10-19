use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::models::*;
use crate::db::Db;

pub fn generate_inward_slip_pdf(slip: &InwardSlip, db: &Db) -> Result<String, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Inward Slip", Mm(80.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let shop_name = db.get_setting("shop_name")?.unwrap_or_default();
    let shop_address = db.get_setting("shop_address")?.unwrap_or_default();
    let shop_phone = db.get_setting("shop_phone")?.unwrap_or_default();
    let shop_email = db.get_setting("shop_email")?.unwrap_or_default();
    let jurisdiction = db.get_setting("jurisdiction")?.unwrap_or_default();

    let mut y = Mm(280.0);

    current_layer.use_text(&shop_name, 14.0, Mm(5.0), y, &font_bold);
    y -= Mm(6.0);
    current_layer.use_text(&shop_address, 8.0, Mm(5.0), y, &font);
    y -= Mm(5.0);
    current_layer.use_text(&format!("Mo: {}  E-mail: {}", shop_phone, shop_email), 8.0, Mm(5.0), y, &font);
    y -= Mm(8.0);

    current_layer.use_text("INWARD SLIP", 12.0, Mm(25.0), y, &font_bold);
    y -= Mm(8.0);

    current_layer.use_text(&format!("Inward No: {}", slip.slip_no), 9.0, Mm(5.0), y, &font);
    current_layer.use_text(&format!("Date: {}", slip.date), 9.0, Mm(50.0), y, &font);
    y -= Mm(6.0);

    if let Some(name) = &slip.customer_name {
        current_layer.use_text(&format!("To: {}", name), 9.0, Mm(5.0), y, &font);
        y -= Mm(5.0);
    }
    if let Some(phone) = &slip.customer_phone {
        current_layer.use_text(&format!("Mobile No: {}", phone), 9.0, Mm(5.0), y, &font);
        y -= Mm(5.0);
    }

    current_layer.use_text(&format!("Device: {}", slip.device_type), 9.0, Mm(5.0), y, &font);
    y -= Mm(8.0);

    current_layer.use_text("Parts Check:", 9.0, Mm(5.0), y, &font_bold);
    y -= Mm(5.0);

    let parts = vec![
        "Model No./MB", "Processor", "Ram", "HDD/SSD",
        "Graphics Card", "SMPS", "Password", "Any Other"
    ];
    for part in parts {
        current_layer.use_text(&format!("  [ ] {}", part), 8.0, Mm(8.0), y, &font);
        y -= Mm(4.5);
    }
    y -= Mm(3.0);

    current_layer.use_text("Problem:", 9.0, Mm(5.0), y, &font_bold);
    y -= Mm(5.0);
    if let Some(problem) = &slip.problem_description {
        current_layer.use_text(problem, 8.0, Mm(8.0), y, &font);
        y -= Mm(5.0);
    }
    y -= Mm(5.0);

    current_layer.use_text("Terms & Conditions:", 8.0, Mm(5.0), y, &font_bold);
    y -= Mm(4.0);
    let terms = vec![
        "1. Data backup is customer's responsibility.",
        "2. Warranty only on replaced parts.",
        "3. 10 days warranty on repairs.",
        "4. Subject to Anand Jurisdiction.",
    ];
    for term in terms {
        current_layer.use_text(term, 7.0, Mm(8.0), y, &font);
        y -= Mm(3.5);
    }
    y -= Mm(10.0);

    current_layer.use_text("Customer Sign: _________________", 8.0, Mm(5.0), y, &font);
    current_layer.use_text("Authorized Signatory", 8.0, Mm(50.0), y, &font);

    let path = format!("/tmp/inward_slip_{}.pdf", slip.slip_no);
    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);
    doc.save(&mut buf)?;
    buf.flush()?;

    Ok(path)
}

pub fn generate_bill_pdf(bill: &Bill, db: &Db) -> Result<String, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Bill", Mm(80.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let shop_name = db.get_setting("shop_name")?.unwrap_or_default();
    let shop_address = db.get_setting("shop_address")?.unwrap_or_default();
    let shop_phone = db.get_setting("shop_phone")?.unwrap_or_default();
    let shop_email = db.get_setting("shop_email")?.unwrap_or_default();
    let jurisdiction = db.get_setting("jurisdiction")?.unwrap_or_default();

    let mut y = Mm(280.0);

    current_layer.use_text(&shop_name, 14.0, Mm(5.0), y, &font_bold);
    y -= Mm(6.0);
    current_layer.use_text(&shop_address, 8.0, Mm(5.0), y, &font);
    y -= Mm(5.0);
    current_layer.use_text(&format!("Mo: {}  E-mail: {}", shop_phone, shop_email), 8.0, Mm(5.0), y, &font);
    y -= Mm(8.0);

    current_layer.use_text("BILL / INVOICE", 12.0, Mm(25.0), y, &font_bold);
    y -= Mm(8.0);

    current_layer.use_text(&format!("Bill No: {}", bill.bill_no), 9.0, Mm(5.0), y, &font);
    current_layer.use_text(&format!("Date: {}", bill.created_at), 9.0, Mm(50.0), y, &font);
    y -= Mm(6.0);

    if let Some(name) = &bill.customer_name {
        current_layer.use_text(&format!("Customer: {}", name), 9.0, Mm(5.0), y, &font);
        y -= Mm(5.0);
    }
    if let Some(phone) = &bill.customer_phone {
        current_layer.use_text(&format!("Phone: {}", phone), 9.0, Mm(5.0), y, &font);
        y -= Mm(5.0);
    }
    y -= Mm(3.0);

    current_layer.use_text("Description", 8.0, Mm(5.0), y, &font_bold);
    current_layer.use_text("Qty", 8.0, Mm(50.0), y, &font_bold);
    current_layer.use_text("Rate", 8.0, Mm(58.0), y, &font_bold);
    current_layer.use_text("Total", 8.0, Mm(68.0), y, &font_bold);
    y -= Mm(5.0);

    for item in &bill.items {
        current_layer.use_text(&item.description, 8.0, Mm(5.0), y, &font);
        current_layer.use_text(&item.quantity.to_string(), 8.0, Mm(50.0), y, &font);
        current_layer.use_text(&format!("{:.2}", item.unit_price), 8.0, Mm(58.0), y, &font);
        current_layer.use_text(&format!("{:.2}", item.total), 8.0, Mm(68.0), y, &font);
        y -= Mm(5.0);
    }
    y -= Mm(3.0);

    current_layer.use_text(&format!("Subtotal: {:.2}", bill.subtotal), 9.0, Mm(50.0), y, &font);
    y -= Mm(5.0);
    current_layer.use_text(&format!("Tax: {:.2}", bill.tax), 9.0, Mm(50.0), y, &font);
    y -= Mm(5.0);
    current_layer.use_text(&format!("Total: {:.2}", bill.total_amount), 10.0, Mm(50.0), y, &font_bold);
    y -= Mm(8.0);

    current_layer.use_text(&format!("Payment: {}", bill.payment_status), 9.0, Mm(5.0), y, &font);
    y -= Mm(10.0);

    current_layer.use_text(&jurisdiction, 7.0, Mm(5.0), y, &font);
    y -= Mm(8.0);
    current_layer.use_text("Authorized Signatory", 8.0, Mm(45.0), y, &font);

    let path = format!("/tmp/bill_{}.pdf", bill.bill_no);
    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);
    doc.save(&mut buf)?;
    buf.flush()?;

    Ok(path)
}

pub fn generate_quotation_pdf(quote: &Quotation, db: &Db) -> Result<String, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Quotation", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let shop_name = db.get_setting("shop_name")?.unwrap_or_default();
    let shop_address = db.get_setting("shop_address")?.unwrap_or_default();
    let shop_phone = db.get_setting("shop_phone")?.unwrap_or_default();
    let shop_email = db.get_setting("shop_email")?.unwrap_or_default();

    let mut y = Mm(275.0);

    current_layer.use_text(&shop_name, 16.0, Mm(10.0), y, &font_bold);
    y -= Mm(7.0);
    current_layer.use_text(&shop_address, 9.0, Mm(10.0), y, &font);
    y -= Mm(6.0);
    current_layer.use_text(&format!("Mo: {}  E-mail: {}", shop_phone, shop_email), 9.0, Mm(10.0), y, &font);
    y -= Mm(10.0);

    current_layer.use_text("QUOTATION", 14.0, Mm(75.0), y, &font_bold);
    y -= Mm(8.0);

    current_layer.use_text(&format!("Quote No: {}", quote.quotation_no), 10.0, Mm(10.0), y, &font);
    current_layer.use_text(&format!("Date: {}", quote.created_at), 10.0, Mm(140.0), y, &font);
    y -= Mm(7.0);

    if let Some(name) = &quote.customer_name {
        current_layer.use_text(&format!("Customer: {}", name), 10.0, Mm(10.0), y, &font);
        y -= Mm(7.0);
    }
    y -= Mm(5.0);

    current_layer.use_text("Description", 9.0, Mm(10.0), y, &font_bold);
    current_layer.use_text("Qty", 9.0, Mm(120.0), y, &font_bold);
    current_layer.use_text("Rate", 9.0, Mm(140.0), y, &font_bold);
    current_layer.use_text("Total", 9.0, Mm(165.0), y, &font_bold);
    y -= Mm(6.0);

    for item in &quote.items {
        current_layer.use_text(&item.description, 9.0, Mm(10.0), y, &font);
        current_layer.use_text(&item.quantity.to_string(), 9.0, Mm(120.0), y, &font);
        current_layer.use_text(&format!("{:.2}", item.unit_price), 9.0, Mm(140.0), y, &font);
        current_layer.use_text(&format!("{:.2}", item.total), 9.0, Mm(165.0), y, &font);
        y -= Mm(6.0);
    }
    y -= Mm(5.0);

    current_layer.use_text(&format!("Total Amount: {:.2}", quote.total_amount), 11.0, Mm(130.0), y, &font_bold);
    y -= Mm(10.0);

    if let Some(notes) = &quote.notes {
        current_layer.use_text(&format!("Notes: {}", notes), 8.0, Mm(10.0), y, &font);
        y -= Mm(6.0);
    }
    y -= Mm(15.0);

    current_layer.use_text("Authorized Signatory", 9.0, Mm(130.0), y, &font);

    let path = format!("/tmp/quotation_{}.pdf", quote.quotation_no);
    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);
    doc.save(&mut buf)?;
    buf.flush()?;

    Ok(path)
}
