use std::env;
use std::net::{ IpAddr, TcpStream };
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{ Sender, channel };
use std::thread;
use std::io::Write;

const MAX : u16 = 10000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let config = Config::new(args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            println!("{} problem parsing arguments: {}", program, err);
            process::exit(1);
        }
    });

    let num_threads = config.threads;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, config.ip_address, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }
    
    for v in out {
        println!("{} is open", v);
    }
}

struct Config {
    threads: u16,
    ip_address: IpAddr,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let threads = 10;

        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let ip = args[1].clone();
        if let Ok(ip_address) = IpAddr::from_str(&ip) {
            Ok(Config {
                threads,
                ip_address,
            })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage: -j to select number of threads, -h for help");
                Err("help")
            } else if flag.contains("-h") || flag.contains("--help") {
               Err("Too many arguments")
            } else if flag.contains("-j") && args.len() == 4 {
                // return config
                let threads = args[2].clone();
                let ip = args[3].clone();

                let ip_address = match IpAddr::from_str(&ip) {
                    Ok(s) => s,
                    Err(_) => Err("Invalid IP address")?,
                };

                let threads = match u16::from_str(&threads) {
                    Ok(s) => s,
                    Err(_) => Err("Kindly enter a valid number of threads")?
                };

                Ok(Config {
                    ip_address,
                    threads,
                })
            } else {
                Err("Invalid syntax")
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port : u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
                tx.send(port).unwrap()
            },

            Err(_) => {

            }
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}