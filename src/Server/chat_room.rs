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
    clients: Vec<Box<ThreadClient>>,
    messages: Receiver<String>,
    sender: Sender<String>
}

impl ChatRoom {
    pub fn new(max_clients: usize) -> ChatRoom {
        let clients = Vec::with_capacity(max_clients);
        let (sender, receiver) = channel();
        ChatRoom{clients: clients,
                 server_running: AtomicBool::new(false),
                 messages: receiver, sender: sender }
    }

    pub fn add_client(&mut self, stream: TcpStream)  -> usize {
        let p_queue = self.sender.clone();
        let mut client = Box::new(ThreadClient::new(stream, p_queue));
        client.start();
        self.clients.push(client);
        return self.clients.len();
    }

    pub fn update_clients(&mut self) {
        for message in self.messages.iter() {
            for client in self.clients.iter_mut() {
                client.send_message(&message);
            }
        }
    }
}