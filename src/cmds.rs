pub use tauri;
#[tauri::command]
pub fn get_ip() -> Vec<String> {
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
#[tauri::command]
pub fn start_udp(mut ip: String, port: u16) {
    ip.push_str(":");
    ip.push_str(&port.to_string());
    std::net::UdpSocket::bind(ip).unwrap();
}
