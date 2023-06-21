// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cryp::rsa;
use num_bigint::BigInt;
use tauri::{ Manager, Window, regex::bytes::Replacer };

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
    let encrypt_msg = rsa::test_verschlüselung("Test".to_string(), &keys.public);
    let decrypt_msg = rsa::test_enschlüselung(encrypt_msg.clone(), &keys.private);
    println!("decrypt_msg.to_signed_bytes_be().as_ref() -> {:?}", decrypt_msg.to_signed_bytes_be());
    println!(
        "crypt_msg is {:#?}\ndecrypt_msg is {:?}",
        bigint_in_readabil(encrypt_msg, 32),
        BigInt::parse_bytes(&decrypt_msg.to_signed_bytes_be(), 10)
    );

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
fn bigint_in_readabil(bigint_msg: BigInt, factor: u32) -> String {
    return bigint_msg.to_str_radix(factor);
}
fn vec_in_readabil(big_int_vec: Vec<BigInt>, factor: u32) -> Vec<String> {
    let mut string_big_int_vec: Vec<String> = Vec::new();
    for zahl in big_int_vec {
        string_big_int_vec.push(zahl.to_str_radix(factor));
    }
    return string_big_int_vec;
}
