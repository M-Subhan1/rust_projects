use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        }));

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size.into());

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            sender,
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub threads: usize
}

impl Config {
    pub fn new (args: Vec<String>) -> Result<Config, &'static str> {
        let mut port = 8000;
        let mut threads = 4;
        
        if args.len() > 4 {
            return Err("Too many arguments");
        }
        
        if args.len() == 2 && args[1].contains("-h") {
            println!("Available Configuration\n\t-h: brings up the current help menu\n\t-p specify port to use\n\t-t specify the number of threads to allocate\n\tNote: If using multiple flags use the following syntax -pt [PORT] [NUM_THREADS]\n.");
            return Err("help");
        } else if args.len() == 3 && args[1].contains("-t") {
            match args[2].parse::<usize>() {
                Ok(num) => threads = num,
                Err(_) => return Err("Invalid thread number")
            }
        } else if args.len() == 3 && args[1].contains("-p") {
            match args[2].parse::<u16>() {
                Ok(num) => port = num,
                Err(_) => return Err("Invalid Port")
            }
        } else if args.len() == 4 && args[1].contains("-pt")  {
            port = args[2].parse::<u16>().unwrap();
            threads = args[3].parse::<usize>().unwrap();
        } 
        
        Ok(Config {
            port,
            threads,
        })
    }   
}
