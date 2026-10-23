#![windows_subsystem = "windows"]

mod db;
mod models;
mod commands;
mod pdf;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));
            if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
                eprintln!("Failed to create app data dir: {}", e);
            }
            let db_path = app_data_dir.join("mann_computer.db");
            eprintln!("Database path: {:?}", db_path);
            let state = commands::AppState::new(db_path).expect("Failed to initialize database");
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_customers,
            commands::create_customer,
            commands::update_customer,
            commands::get_inward_slips,
            commands::get_inward_slip,
            commands::create_inward_slip,
            commands::update_inward_slip_status,
            commands::get_quotations,
            commands::create_quotation,
            commands::get_repairs,
            commands::create_repair,
            commands::update_repair,
            commands::get_bills,
            commands::create_bill,
            commands::update_bill_payment,
            commands::get_parts,
            commands::create_part,
            commands::update_part_stock,
            commands::get_part_sales,
            commands::create_part_sale,
            commands::get_cctv_quotes,
            commands::create_cctv_quote,
            commands::get_laptop_quotes,
            commands::create_laptop_quote,
            commands::create_delivery,
            commands::get_dashboard_stats,
            commands::get_analytics,
            commands::backup_database,
            commands::restore_database,
            commands::export_settings,
            commands::import_settings,
            commands::generate_pdf,
            commands::get_settings,
            commands::update_setting,
            commands::health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
