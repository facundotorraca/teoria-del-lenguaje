use std::thread;
use std::net::TcpStream;
use bufstream::BufStream;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::io::{Read, Write, BufRead, BufWriter, BufReader};
use std::time::Duration;

enum ThreadHandler {
    JoinHandler(thread::JoinHandle<()>), //Thread running
    Nil //Thread not spawned
}

pub struct ThreadClient {
    username: String,
    handler: ThreadHandler,
    messages: Arc<Mutex<VecDeque<String>>>,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>
}

fn run(reader: Arc<Mutex<BufReader<TcpStream>>>, messages: Arc<Mutex<VecDeque<String>>>, username: &String) {
    let username_prefix = String::from(username.to_uppercase().to_owned() + ": ");

    let mut buffer = String::new();
    loop {
        reader.lock().unwrap().read_line(&mut buffer);
        let incoming_message = String::from(username_prefix.to_owned() + &buffer.to_owned() ).replace("\n", "");
        messages.lock().unwrap().push_back(incoming_message);
        buffer.clear();
    }
}

impl ThreadClient {
    pub fn new(stream: TcpStream, p_queue: Arc<Mutex<VecDeque<String>>>) -> ThreadClient {
        let stream_clone = stream.try_clone().unwrap();

        ThreadClient{ handler: ThreadHandler::Nil,
                      username: String::new(),
                      writer: Arc::new(Mutex::new(BufWriter::new(stream))),
                      reader: Arc::new(Mutex::new(BufReader::new(stream_clone))),
                      messages: p_queue }
    }

    pub fn start(&mut self) {
        let mut new_username = String::new();
        self.reader.lock().unwrap().read_line(&mut new_username).unwrap();
        new_username = new_username.replace("\n", "");
        self.writer.lock().unwrap().write_fmt(format_args!("OK\n"));
        self.writer.lock().unwrap().flush();

        self.writer.lock().unwrap().write_fmt(format_args!("Welcome {} to the FaceRust CHAT\n", new_username));
        self.writer.lock().unwrap().flush();

        self.username = String::from(&new_username);

        let mut reader_clone = self.reader.clone();
        let mut messages_clone = self.messages.clone();

        let handler =  thread::spawn(move || { run(reader_clone, messages_clone, &new_username); });

        self.handler = ThreadHandler::JoinHandler(handler);
    }

    pub fn send_message(&mut self, message: &String) {
        let end_name = message.find(":").unwrap();
        if !&message[0..end_name].eq(&self.username.to_uppercase())  {
            self.writer.lock().unwrap().write_fmt( format_args!("{}\n", &message) );
            self.writer.lock().unwrap().flush();
        }
        println!("MSG ENVIADO A: {}", &message[0..end_name]);
    }
}