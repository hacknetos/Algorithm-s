// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cryp::rsa;
use tauri::{ Manager, Window };

mod cryp;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn gen_key(window: Window) -> Vec<String> {
    let keys = rsa::GenerateKeys(window);

    let mut res: Vec<String> = Vec::new();
    let mut str = String::from(
        keys.public.n.to_str_radix(16) + "," + &keys.public.e.to_str_radix(16)
    );
    res.push(str);
    str = String::from(
        keys.private.p.to_str_radix(16) +
            "," +
            &keys.private.q.to_str_radix(16) +
            "," +
            &keys.private.d.to_str_radix(16)
    );
    res.push(str);

    return res;
}
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, gen_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
