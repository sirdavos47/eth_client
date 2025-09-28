use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_p2p_server(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("Port dinlenemedi");
    println!("P2P sunucusu {} portunda başlatıldı", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Bağlantı hatası: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Gelen veri: {}", String::from_utf8_lossy(&buffer[..size]));
            let _ = stream.write(b"Merhaba, peer!");
        }
        Err(e) => {
            eprintln!("Okuma hatası: {}", e);
        }
    }
}
