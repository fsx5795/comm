#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cmds;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::cmds::get_ip, crate::cmds::start_udp])
        .run(tauri::generate_context!()).expect("error");
}
