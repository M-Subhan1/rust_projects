use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;

mod server;
use server::ThreadPool;

fn main() {
    let config: server::Config = server::Config::new(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let thread_pool = ThreadPool::new(config.threads);
    let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", config.port))
        .unwrap_or_else(|err| {
            eprintln!("Problem binding to port: {}", err);
            std::process::exit(1);
        });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => thread_pool.execute(|| {handle_connection(stream)}),
            Err(e) => eprintln!("Problem handling connection: {}", e)
        }
    }
    
    println!("{:?}", &config);
}

fn handle_connection(mut stream : TcpStream) {
    let mut buffer: Vec<u8> = vec![0;1024];
    stream.read(&mut buffer).unwrap();

    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
    }

    let (status_line, filename) = if buffer.starts_with(get) || buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}