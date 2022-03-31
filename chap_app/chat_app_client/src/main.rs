use std::net::{TcpStream};
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;

fn main() {
    let mut client = TcpStream::connect(format!("{}:{}", "127.0.0.1", 80)).expect("Could not connect to server");
    let mut buffer = vec![0;1024];
    let message = b"Hello from the client";
    
    client.write(message).expect("Could not send message to server");
    client.flush().expect("Could not flush message to server");
    
    client.read(&mut buffer).expect("Could not read from server");
    println!("Message From Server: {}", String::from_utf8_lossy(&buffer));

    thread::sleep(Duration::from_secs(2));
    
}