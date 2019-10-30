use std::net::TcpStream;
use std::sync::{Mutex, Arc};
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use crate::thread_client::ThreadClient;
use std::thread;
use std::time::Duration;

pub struct ChatRoom {
    server_running: AtomicBool,
    clients: Vec<Box<ThreadClient>>,
    messages: Arc<Mutex<VecDeque<String>>>
}

impl ChatRoom {
    pub fn new(max_clients: usize) -> ChatRoom {
        let clients = Vec::with_capacity(max_clients);
        let mut messages : VecDeque<String> = VecDeque::new();

        ChatRoom{clients: clients,
                 server_running: AtomicBool::new(false),
                 messages: Arc::new(Mutex::new(messages)) }
    }

    pub fn add_client(&mut self, stream: TcpStream)  -> usize {
        let messages_clone = self.messages.clone();
        let mut client = Box::new(ThreadClient::new(stream, messages_clone));
        client.start();
        self.clients.push(client);
        return self.clients.len();
    }

    pub fn update_clients(&mut self) {
        while !self.messages.lock().unwrap().is_empty() {
            let message = self.messages.lock().unwrap().pop_front().unwrap();
            for client in self.clients.iter_mut() {
                client.send_message(&message);
            }
        }
    }
}