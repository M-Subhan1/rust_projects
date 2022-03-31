use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", 80))
        .unwrap_or_else(|err| {
            eprintln!("Problem binding to port: {}", err);
            std::process::exit(1);
        });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Problem handling connection: {}", e)
        }
    }
}

fn handle_connection (mut stream: TcpStream) {
    let mut buffer: Vec<u8> = vec![0;1024];

    println!("Connection established!");
    stream.read(&mut buffer).unwrap();

    let message = b"Hello from the server!";
    stream.write(message).expect("Could not write to the client");
    stream.flush().expect("Could not flush to the client");

    println!("Message Received: {}", String::from_utf8_lossy(&buffer));
}