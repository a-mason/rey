#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod prisma;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let client_result: Result<PrismaClient, NewClientError> = prisma::new_client().await;
    let _ = client_result.expect("Failed to setup prisma client");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
            println!("Setting up");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
