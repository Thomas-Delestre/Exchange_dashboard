use dotenv::dotenv;
use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
    let api_key = get_api_key();
    println!("Clé API : {}", api_key);
    println!("greet catch");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_api_key() -> String {
    println!("get_api catch");
    dotenv().ok(); // Charge le fichier .env
    env::var("ALPHA_VANTAGE_API_KEY").expect("La clé API n'est pas définie")
}