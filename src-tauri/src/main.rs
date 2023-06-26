// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ rc::Rc };
use std::env;
use cryp::converter::{ dec_to_hex, hex_to_dec, dec_to_base32, base32_to_dec, self };
use cryp::rsa::{ self, Keys };
use num::BigInt;
use tauri::{ Window };

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
async fn gen_key(window: Window) -> (Vec<Vec<std::string::String>>, String, String, String) {
    let keys = rsa::generate_keys(window.clone());

    let mut limetet_text_vec = text_limiter(keys.public.n.to_str_radix(32), 64);
    limetet_text_vec.append(&mut text_limiter(keys.public.e.to_str_radix(32), 64));

    let mut res: Vec<Vec<String>> = Vec::new();
    res.push(limetet_text_vec);

    let mut limetet_text_vec = text_limiter(keys.private.p.to_str_radix(32), 64);
    limetet_text_vec.append(&mut text_limiter(keys.private.q.to_str_radix(32), 64));
    limetet_text_vec.append(&mut text_limiter(keys.private.d.to_str_radix(32), 64));

    res.push(limetet_text_vec);

    let _ = window.emit("RSA-Stap", Payload {
        waht: "Done".into(),
        stap: 7,
        from: 7,
    });

    return (
        res,
        converter::dec_to_base32(keys.public.n),
        converter::dec_to_base32(keys.public.e),
        converter::dec_to_base32(keys.private.d),
    );
}

#[tauri::command]
async fn encrypt_msg(window: tauri::Window, msg: String, key: (String, String)) -> String {
    let opt = rsa::encrypt(converter::base32_to_dec(key.0), converter::base32_to_dec(key.1), msg);
    if opt.is_none() {
        return String::from("The Msg Must Smoler then N");
    } else {
        return dec_to_base32(opt.unwrap());
    }
}

#[tauri::command]
async fn dectypt_msg(window: tauri::Window, msg: String, key: (String, String)) -> String {
    let de_msg = rsa
        ::dectypt(converter::base32_to_dec(key.0), converter::base32_to_dec(key.1), msg)
        .to_signed_bytes_le();
    return String::from_utf8(de_msg).unwrap();
}
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, gen_key, encrypt_msg, dectypt_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn text_limiter(text_to_space: String, how_long: u32) -> Vec<String> {
    let mut counter = 0;
    let mut spacet_text = String::new();
    let mut result: Vec<String> = Vec::new();
    for buchstabe in text_to_space.trim().chars() {
        if counter == how_long {
            result.push(spacet_text);
            spacet_text = String::new();
            counter = 0;
        }
        spacet_text = spacet_text + buchstabe.to_string().as_str();
        counter += 1;
    }
    result.push(spacet_text);
    return result.to_owned();
}
