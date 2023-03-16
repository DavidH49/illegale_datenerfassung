// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_handler;
use tauri::generate_context;
use rusqlite::{Connection, Result};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn gather_info(fname: &str, lname: &str, mail: &str, pnum: &str, gen: &str, bday: &str, balls: &str, panzerschorle: bool) -> String {

    // If balls is not a number, default it to 0
    let balls: u8 = match balls.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };


    // Creates the user object
    let u = User {
        first_name: fname.to_string(),
        last_name: lname.to_string(),
        email: mail.to_string(),
        phone: pnum.to_string(),
        gender: gen.to_string(),
        birth: bday.to_string(),
        testicles: balls,
        drinks_panzerschorle: if panzerschorle { 1 } else { 0 }
    };

    // Inserts the user into users_db.db
    let conn = connect_db().unwrap();
    send_user_data
        (&conn, &u)
        .expect("Data couldn't be sent!");


    // Just for debug purposes, might remove that when db works
    println!("Name: {} {}\nEmail Adress: {}\nPhone Number: {}\nGender: {}\nBirthdate: {}\nTesticles: {}\nDrinks Panzerschorle: {}",
             u.first_name, u.last_name, u.email, u.phone, u.gender, u.birth, u.testicles, u.drinks_panzerschorle);

    // If the user doesn't drink Panzerschorle, put them on the hit list.
    if panzerschorle {
        format!("Vielen Dank für Ihre Informationen.")
    } else {
        add_to_hitlist(&conn);
        format!("Kaufen Sie sich sofort eine Panzerschorle bei Ihrer lokalen Tankstelle oder Supermarkt, sonst kommen wir zu Ihnen nachhause!")
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![gather_info])
        .run(generate_context!())
        .expect("error while running tauri application");
}



fn connect_db() -> Result<Connection> {
    let db_path = "./../users_db.db";
    let conn = Connection::open(db_path)?;

    Ok(conn)
}

fn send_user_data(conn: &Connection, u: &User) -> Result<()> {
    let query: &str = "INSERT INTO User (FirstName, LastName, EMailAddress, PhoneNumber, Gender, BirthDate, Testicles, DrinksPanzerschorle) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";

    let u = [&u.first_name, &u.last_name, &u.email, &u.phone, &u.gender, &u.birth, &u.testicles.to_string(), &u.drinks_panzerschorle.to_string()];

    conn.execute
        (query, u)
        .expect("Data couldn't be sent!");

    Ok(())
}

fn add_to_hitlist(conn: &Connection) {
    let query = "INSERT INTO HitList (User) VALUES (?1)";

    conn.execute
        (query, [conn.last_insert_rowid()])
        .expect("The user couldn't be added to the hit list.");
}


struct User {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    gender: String,
    birth: String,
    testicles: u8,
    drinks_panzerschorle: u8
}

/* Genders: Male, Female, Diverse, Cowboysfromhellgender
enum Gender {
    M,
    F,
    D,
    CFH
}
*/