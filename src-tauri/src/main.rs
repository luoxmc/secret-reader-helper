#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::net::TcpStream;

#[tauri::command]
async fn send_tcp_msg(msg: String) {
    let stream = TcpStream::connect("127.0.0.1:7663").await.unwrap();
    stream.writable().await.unwrap();
    stream.try_write(msg.as_bytes()).unwrap();
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .system_tray(tauri::SystemTray::default())
        .invoke_handler(tauri::generate_handler![greet,send_tcp_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
