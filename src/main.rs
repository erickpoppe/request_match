use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    match stream.read(&mut buf) {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);
            println!("Received request: {}", request);
            match request.as_ref() {
                "GET /hello HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n" => {
                    let response = "HTTP/1.1 200 OK\r\nHello, world!";
                    stream.write_all(response.as_bytes()).unwrap();
                },
                _ => {
                    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        },
        Err(e) => {
            println!("Error reading from socket: {}",
                     e);
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());
                std::thread::spawn(|| { handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting client:{}", e);
            }
        }
    }
}