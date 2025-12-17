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
        .map(|_| chars_iter.clone().choose(&mut rand::rng()).unwrap())
        .collect();

    return password;
}

#[tauri::command]
fn open_github() -> () {
    open::that("https://github.com/gage-lodba/SafeGen").expect("Failed to open GitHub page");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![generate_password, open_github])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
