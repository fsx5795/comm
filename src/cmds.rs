pub use tauri;
use tauri::Emitter;
#[tauri::command]
pub fn get_ip() -> Vec<String> {
    let interfaces = local_ip_address::list_afinet_netifas().unwrap();
    interfaces.iter().filter(|(_, ip)| ip.to_string().len() > 10 && ip.to_string().len() <= 16).map(|(_, ip)|  ip.to_string()).collect::<Vec<String>>()
}
#[tauri::command]
pub fn start_udp(ip: String, port: u16, multaddr: String, handle: tauri::AppHandle) {
    let socket = match std::net::UdpSocket::bind(format!("{}:{}", ip, port)) {
        Ok(socket) => socket,
        Err(err) => {
            handle.emit_to("main", "error", err.to_string()).unwrap();
            return;
        }
    };
    socket.set_broadcast(true).unwrap();
    socket.set_multicast_loop_v4(false).unwrap();
    if !multaddr.is_empty() {
        let multaddr = multaddr.split('.').map(|s| s.parse().unwrap()).collect::<Vec<u8>>();
        socket.join_multicast_v4(&std::net::Ipv4Addr::new(multaddr[0], multaddr[1], multaddr[2], multaddr[3]), &std::net::Ipv4Addr::new(0, 0, 0, 0)).unwrap();
    }
    std::thread::spawn(move || {
        loop {
            let mut buf = [0; 3600];
            let (amt, addr) = socket.recv_from(&mut buf).unwrap();
            println!("{}{}", addr.to_owned(), String::from_utf8_lossy(&buf[..amt]).to_owned());
        }
    });
}
