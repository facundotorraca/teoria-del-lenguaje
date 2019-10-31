use std::net::TcpStream;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use crate::thread_client::ThreadClient;
use std::thread;
use std::time::Duration;

pub struct ChatRoom {
    server_running: AtomicBool,
    clients: Mutex<Vec<Box<ThreadClient>>>,
    messages: Mutex<Receiver<String>>,
    sender: Mutex<Sender<String>>
}

impl ChatRoom {
    pub fn new(max_clients: usize) -> ChatRoom {
        let clients = Mutex::new(Vec::with_capacity(max_clients));
        let (sender, receiver) = channel();
        let receiver = Mutex::new(receiver);
        let sender = Mutex::new(sender);
        ChatRoom{clients: clients,
                 server_running: AtomicBool::new(false),
                 messages: receiver, sender: sender }
    }

    pub fn add_client(&self, stream: TcpStream) -> usize {
        let p_queue = self.sender.lock().unwrap().clone();
        let mut client = Box::new(ThreadClient::new(stream, p_queue));
        client.start();
        self.clients.lock().unwrap().push(client);
        return self.clients.lock().unwrap().len();
    }

    pub fn update_clients(&self) {
        for message in self.messages.lock().unwrap().iter() {
            for client in self.clients.lock().unwrap().iter_mut() {
                client.send_message(&message);
            }
        }
    }
}