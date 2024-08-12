// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn generate_password(length: u8, lower: bool, upper: bool, number: bool, symbol: bool) -> String {
    use rand::seq::IteratorRandom;

    let lower_set: &str = "abcdefghijklmnopqrstuvwxyz";
    let upper_set: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let number_set: &str = "0123456789";
    let symbol_set: &str = "!@#$%^&*";

    let chars_iter = [
        (lower_set, lower),
        (upper_set, upper),
        (number_set, number),
        (symbol_set, symbol),
    ]
    .into_iter()
    .filter(|(_set, include)| *include)
    .flat_map(|(set, _include)| set.chars());

    let password = (0..length)
        .map(|_| chars_iter.clone().choose(&mut rand::thread_rng()).unwrap())
        .collect();

    return password;
}

#[tauri::command]
fn open_github() -> () {
    let _ = open::that("https://github.com/JerimiahOfficial");
}

#[tauri::command]
fn copy_to_clipboard(_string: String) -> () {
    println!("This command is currently W.I.P")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_password,
            open_github,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
