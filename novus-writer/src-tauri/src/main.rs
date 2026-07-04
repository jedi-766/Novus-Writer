// Tauri application entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use novus_writer_lib::{commands, services, database::Database};
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("novus_writer=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting Novus Writer...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(Mutex::new(Database::new())))
        .invoke_handler(tauri::generate_handler![
            commands::document::create_document,
            commands::document::open_document,
            commands::document::save_document,
            commands::document::delete_document,
            commands::document::list_documents,
            commands::document::rename_document,
            commands::editor::insert_text,
            commands::editor::format_text,
            commands::editor::insert_image,
            commands::editor::insert_table,
            commands::search::find_text,
            commands::search::replace_text,
            commands::export::export_pdf,
            commands::export::export_docx,
            commands::export::export_markdown,
            commands::export::export_html,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
