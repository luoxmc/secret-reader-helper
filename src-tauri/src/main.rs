#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::net::TcpStream;
use std::io;

#[tauri::command]
async fn send_tcp_msg() -> String {
    let stream = TcpStream::connect("127.0.0.1:7663").await.unwrap();
    stream.writable().await.unwrap();
    return match stream.try_write(b"type=4&content=next") {
        Ok(n) => {
            "send success".to_string()
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            "send fail".to_string()
        }
        Err(e) => {
            "err".to_string()
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,send_tcp_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
