use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    match stream.read(&mut buf) {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);
            println!("Received request: {}", request);

            // Split the request into lines and get the first line (request line)
            let mut lines = request.lines();
            if let Some(request_line) = lines.next() {
                // Match only the request line
                match request_line {
                    "GET /hello HTTP/1.1" => {
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            println!("Error writing response: {}", e);
                        }
                        if let Err(e) = stream.flush() {
                            println!("Error flushing stream: {}", e);
                        }
                        println!("Sent response: Hello, world!");
                    },
                    _ => {
                        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            println!("Error writing response: {}", e);
                        }
                        if let Err(e) = stream.flush() {
                            println!("Error flushing stream: {}", e);
                        }
                        println!("Sent response: 404 Not Found");
                    }
                }
            }
        },
        Err(e) => {
            println!("Error reading from socket: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting client: {}", e);
            }
        }
    }
}
