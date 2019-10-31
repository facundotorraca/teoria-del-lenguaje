use std::thread;
use std::net::TcpStream;
use bufstream::BufStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::collections::VecDeque;
use std::io::{Read, Write, BufRead, BufWriter, BufReader};
use std::time::Duration;

fn run(messages: Sender<String>, username: String, reader: Arc<Mutex<BufReader<TcpStream>>>) {
    let username_prefix = String::from(username.to_uppercase().to_owned() + ": ");

    let mut buffer = String::new();
    'client_loop: loop {
        match reader.lock().unwrap().read_line(&mut buffer){
            Ok(_) => {},
            _ => {break 'client_loop},
        }
        let incoming_message = String::from(username_prefix.to_owned() + &buffer.to_owned() ).replace("\n", "");
        messages.send(incoming_message);
        buffer.clear();
    }
}

enum ThreadHandler {
    JoinHandler(thread::JoinHandle<()>), //Thread running
    Nil //Thread not spawned
}

pub struct ThreadClient {
    username: String,
    handler: ThreadHandler,
    messages: Sender<String>,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>
}


impl ThreadClient {
    pub fn new(stream: TcpStream, p_queue: Sender<String>) -> ThreadClient {
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

        let msg_clone = self.messages.clone();
        let user_clone = self.username.clone();
        let reader_clone = self.reader.clone();

        let handler =  thread::spawn(move || { run(msg_clone, user_clone, reader_clone); });

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