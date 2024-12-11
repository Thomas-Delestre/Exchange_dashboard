#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Define the function you want to call from React
#[tauri::command]
fn greet(name: &str) -> String {
    println!("coucou");
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}