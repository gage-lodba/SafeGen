use rand::TryRngCore;
use rand::rngs::OsRng;
use rand::seq::{IndexedRandom, SliceRandom};

const GITHUB_URL: &str = "https://github.com/gage-lodba/SafeGen";

#[tauri::command]
fn generate_password(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String, String> {
    let sets: [(&str, bool); 4] = [
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", upper),
        ("abcdefghijklmnopqrstuvwxyz", lower),
        ("0123456789", number),
        ("!@#$%^&*", symbol),
    ];

    let selected: Vec<Vec<char>> = sets
        .iter()
        .filter(|(_, on)| *on)
        .map(|(s, _)| s.chars().collect())
        .collect();

    if selected.is_empty() {
        return Err("Select at least one character set.".into());
    }
    if (length as usize) < selected.len() {
        return Err(format!(
            "Length must be at least {} to include each selected set.",
            selected.len()
        ));
    }

    let pool: Vec<char> = selected.iter().flatten().copied().collect();
    let mut rng = OsRng.unwrap_err();

    let mut password: Vec<char> = selected
        .iter()
        .map(|set| *set.choose(&mut rng).unwrap())
        .collect();

    for _ in selected.len()..length as usize {
        password.push(*pool.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);

    Ok(password.into_iter().collect())
}

#[tauri::command]
fn open_github() -> Result<(), String> {
    open::that(GITHUB_URL).map_err(|e| format!("Failed to open GitHub page: {e}"))
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![generate_password, open_github])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_no_sets() {
        assert!(generate_password(20, false, false, false, false).is_err());
    }

    #[test]
    fn rejects_length_below_set_count() {
        assert!(generate_password(2, true, true, true, true).is_err());
    }

    #[test]
    fn produces_requested_length() {
        let pw = generate_password(30, true, true, true, true).unwrap();
        assert_eq!(pw.chars().count(), 30);
    }

    #[test]
    fn includes_each_selected_class() {
        let pw = generate_password(20, true, true, true, true).unwrap();
        assert!(pw.chars().any(|c| c.is_ascii_uppercase()));
        assert!(pw.chars().any(|c| c.is_ascii_lowercase()));
        assert!(pw.chars().any(|c| c.is_ascii_digit()));
        assert!(pw.chars().any(|c| "!@#$%^&*".contains(c)));
    }

    #[test]
    fn excludes_unselected_classes() {
        let pw = generate_password(40, false, true, false, false).unwrap();
        assert!(pw.chars().all(|c| c.is_ascii_lowercase()));
    }
}
