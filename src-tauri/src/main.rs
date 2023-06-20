// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cryp::rsa;
use tauri::{ Manager, Window };

mod cryp;
#[derive(Clone, serde::Serialize)]
struct Payload {
    waht: String,
    stap: u32,
    from: u32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn gen_key(window: Window) -> Vec<Vec<String>> {
    let keys = rsa::generate_keys(window.clone());

    let mut res: Vec<Vec<String>> = Vec::new();
    let mut str1 = String::from(keys.public.n.to_str_radix(32));
    let mut str2 = String::from(keys.public.e.to_str_radix(32));
    let mut counter = 0;
    let mut tmp = String::new();
    let mut tmp_vec: Vec<String> = Vec::new();
    for buchstabe in str1.trim().chars() {
        if counter == 64 {
            tmp_vec.push(tmp.clone());
            tmp = String::new();
            counter = 0;
        }
        tmp = tmp + buchstabe.to_string().as_str();
        counter += 1;
    }
    tmp_vec.push(tmp.clone());
    tmp = String::new();
    counter = 0;
    for buchstabe in str2.trim().chars() {
        if counter == 64 {
            tmp_vec.push(tmp);
            tmp = String::new();
            counter = 0;
        }
        tmp = tmp + buchstabe.to_string().as_str();
        counter += 1;
    }
    tmp_vec.push(tmp.clone());
    res.push(tmp_vec);

    tmp = String::new();
    tmp_vec = Vec::new();
    str1 = String::from(keys.private.q.to_str_radix(32));
    str2 = String::from(keys.private.d.to_str_radix(32));
    let mut str3 = String::from(keys.private.p.to_str_radix(32));
    for buchstabe in str1.trim().chars() {
        if counter == 64 {
            tmp_vec.push(tmp.clone());
            tmp = String::new();
            counter = 0;
        }
        tmp = tmp + buchstabe.to_string().as_str();
        counter += 1;
    }
    tmp_vec.push(tmp.clone());
    tmp = String::new();
    counter = 0;
    for buchstabe in str2.trim().chars() {
        if counter == 64 {
            tmp_vec.push(tmp.clone());
            tmp = String::new();
            counter = 0;
        }
        tmp = tmp + buchstabe.to_string().as_str();
        counter += 1;
    }
    tmp_vec.push(tmp.clone());
    tmp = String::new();
    counter = 0;
    for buchstabe in str3.trim().chars() {
        if counter == 64 {
            tmp_vec.push(tmp.clone());
            tmp = String::new();
            counter = 0;
        }
        tmp = tmp + buchstabe.to_string().as_str();
        counter += 1;
    }
    tmp_vec.push(tmp);
    res.push(tmp_vec);
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Done".into(),
        stap: 7,
        from: 7,
    });
    println!("{:?}", res);
    return res;
}
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, gen_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
