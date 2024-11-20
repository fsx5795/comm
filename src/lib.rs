mod cmds;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::cmds::get_ip, crate::cmds::start_udp])
        .run(tauri::generate_context!()).expect("error");
}
