#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn get_ip() -> Vec<String> {
    let mut ips = Vec::new();
    let interfaces = local_ip_address::list_afinet_netifas().unwrap();
    for (_, ip) in interfaces.iter() {
        let ipstr = ip.to_string();
        if ipstr.len() > 10 && ipstr.len() <= 16 {
            ips.push(ipstr);
        }
    }
    ips
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ip])
        .run(tauri::generate_context!()).expect("error");
}
