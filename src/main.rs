#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn get_ip() -> String {
    local_ip_address::local_ip().unwrap().to_string()
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ip])
        .run(tauri::generate_context!()).expect("error");
}
