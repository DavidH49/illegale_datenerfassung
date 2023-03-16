// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_handler;
use tauri::generate_context;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn gather_info(fname: &str, lname: &str, mail: &str, pnum: &str, gen: &str, bday: &str, balls: &str) -> String {
    let user = User {
        first_name: fname.to_string(),
        last_name: lname.to_string(),
        email: mail.to_string(),
        phone: pnum.to_string(),
        gender: gen.to_string(),
        birth: bday.to_string(),
        testicles: balls.trim().parse().unwrap()
    };

    println!("Name: {} {}\nEmail Adress: {}\nPhone Number: {}\nGender: {}\nBirthdate: {}\nTesticles: {}",
             user.first_name, user.last_name, user.email, user.phone, user.gender, user.birth, user.testicles);

    format!("Thank you for your user info.")
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![gather_info])
        .run(generate_context!())
        .expect("error while running tauri application");
}




struct User {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    gender: String,
    birth: String,
    testicles: u8
}

/*
enum Gender {
    M,
    F,
    D,
    CFH
}
*/