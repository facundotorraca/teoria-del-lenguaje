use std::{thread, io};
use bufstream::BufStream;
use std::net::{TcpStream};
use std::sync::{Mutex, Arc, RwLock};
use std::collections::VecDeque;
use std::io::{Read, Write, BufRead, BufWriter, BufReader};
use std::sync::atomic::AtomicBool;

enum ThreadHandler {
    JoinHandler(thread::JoinHandle<()>), //Thread running
    Nil //Thread not spawned
}

enum Username {
    Username(String), //Thread running
    Nil //Thread not spawned
}

pub struct Client {
    username: String,
    running: AtomicBool,
    handler: ThreadHandler,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>,
}

fn run(writer: Arc<Mutex<BufWriter<TcpStream>>>, username: String) {
    let mut input_buffer = String::new();

    loop {
        io::stdin().read_line(&mut input_buffer);
        input_buffer = input_buffer.replace("\n", "");

        writer.lock().unwrap().write_fmt(format_args!("{}\n", &input_buffer));
        writer.lock().unwrap().flush();

        input_buffer.clear();
    }
}

impl Client {
    pub fn new(host: &str, port: &str) -> Client {
        let addr = host.to_owned() + ":" + port;

        let stream = TcpStream::connect(&addr).unwrap();

        let stream_clone = stream.try_clone().unwrap();

        Client{username: String::new(),
               handler: ThreadHandler::Nil,
               writer: Arc::new(Mutex::new(BufWriter::new(stream))),
               reader: Arc::new(Mutex::new(BufReader::new(stream_clone))),
               running: AtomicBool::new(false) }
    }

    pub fn set_username(&mut self, username: &String) -> bool {
        self.writer.lock().unwrap().write_fmt(format_args!("{}\n", &username) ).unwrap();
        self.writer.lock().unwrap().flush().unwrap();

        let mut answer = String::new();
        self.reader.lock().unwrap().read_line(&mut answer).unwrap();
        answer.replace("\n", "");

        if !answer.eq(&String::from("OK\n")) {
            return false;
        }

        self.username = String::from(username).to_uppercase();

        let mut welcome_message = String::new();
        self.reader.lock().unwrap().read_line(&mut welcome_message);

        println!("/*----------------CONNECTION SUCCEED-------------------*/");
        println!("{}", welcome_message.replace("\n", "") );

        return true;
    }

    pub fn start(&mut self) {
        let mut queue: VecDeque<&String> = VecDeque::new();
        let protected_queue = Arc::new(Mutex::new(queue));

        let writer_clone = self.writer.clone();

        let username_copy = self.username.clone();

        let handler =  thread::spawn(move || { run(writer_clone, username_copy); });
        *self.running.get_mut() = true;

        let mut msg_buffer = String::new();

        while *self.running.get_mut() {
            self.reader.lock().unwrap().read_line(&mut msg_buffer);
            msg_buffer = msg_buffer.replace("\n", "");
            println!("{}", msg_buffer);
            msg_buffer.clear();
        }
        self.handler = ThreadHandler::JoinHandler(handler);
    }
}