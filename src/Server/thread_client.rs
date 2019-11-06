use std::thread;
use std::net::{TcpStream, Shutdown};
use bufstream::BufStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::collections::VecDeque;
use std::time::Duration;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{Read, Write, BufRead, BufWriter, BufReader, Error};
use std::borrow::BorrowMut;


fn run(reader: Arc<Mutex<BufReader<TcpStream>>>, messages: Sender<String>, username: String, dead: Arc<AtomicBool>) {
    let username_prefix = String::from(username.to_uppercase().to_owned() + ": ");

    let mut buffer = String::new();
    loop {
        match reader.lock().unwrap().read_line(&mut buffer) {
            Err(_) => { println!("Socket ERROR");
                        dead.store(true, Ordering::Relaxed);
                        break; }

            Ok(0) => {  println!("Client disconnected");
                        dead.store(true, Ordering::Relaxed);
                        break; },

            Ok(_) => {  let incoming_message = String::from(username_prefix.to_owned() + &buffer.to_owned() ).replace("\n", "");
                        messages.send(incoming_message).unwrap();
                        buffer.clear();
                      }
        }

    }
}

pub struct ThreadClient {
    dead: Arc<AtomicBool>,
    username: String,
    stream: TcpStream,
    handler: Option<thread::JoinHandle<()>>,
    messages: Sender<String>,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>
}

impl ThreadClient {
    pub fn new(stream: TcpStream, p_queue: Sender<String>) -> ThreadClient {
        let stream_clone_reader = stream.try_clone().unwrap();
        let stream_clone_writer = stream.try_clone().unwrap();

        ThreadClient{ handler: None,
                      stream: stream,
                      username: String::new(),
                      dead: Arc::new(AtomicBool::new(false)),
                      writer: Arc::new(Mutex::new(BufWriter::new(stream_clone_writer))),
                      reader: Arc::new(Mutex::new(BufReader::new(stream_clone_reader))),
                      messages: p_queue }
    }

    fn get_username(&mut self) {
        let mut new_username = String::new();
        self.reader.lock().unwrap().read_line(&mut new_username).unwrap();
        new_username = new_username.replace("\n", "");

        /* Send OK if username is available */
        self.writer.lock().unwrap().write_fmt(format_args!("OK\n"));
        self.writer.lock().unwrap().flush();

        self.writer.lock().unwrap().write_fmt(format_args!("Welcome {} to the FaceRust CHAT\n", &self.username));
        self.writer.lock().unwrap().flush();

        self.username = String::from(&new_username);
    }

    pub fn start(&mut self) {
        self.get_username();

        let reader_clone = self.reader.clone();
        let messages_clone = self.messages.clone();
        let username_clone = self.username.clone();
        let dead_clone = self.dead.clone();

        let handler =  thread::spawn(move || { run(reader_clone, messages_clone, username_clone, dead_clone); });

        /*Type Option represents an optional
          value: every Option is either Some and
          contains a value, or None, and does not.
          Option types are very common in Rust code,
          as they have a number of uses */

         self.handler = Option::from(handler);
    }

    pub fn send_message(&mut self, message: &String) {
        let end_name = message.find(":").unwrap();

            /*Se envia el mensaje a todos menos al que lo envio*/
        if !&message[0..end_name].eq(&self.username.to_uppercase()) {
            /*This function internally uses the write_all*/
            match self.writer.lock().unwrap().write_fmt(format_args!("{}\n", &message)) {
                Err(_) => { self.dead.store(true, Ordering::Relaxed); }
                _ => {}
            }
            match self.writer.lock().unwrap().flush() {
                Err(_) => { self.dead.store(true, Ordering::Relaxed); }
                Ok(_) => {}
            }
        }
    }

    pub fn join(&mut self) {
        /*Takes the value out of the option, leaving a None in its place.*/
        if let Some(handle) = self.handler.take() {
            handle.join().expect("Failed to join thread");
        }
    }

    pub fn close(&mut self) {
        self.stream.shutdown((Shutdown::Both)).expect("shutdown call failed");
        self.join();
    }

    pub fn is_alive(&self) -> bool {
        return !self.dead.load(Ordering::Relaxed);
    }
}