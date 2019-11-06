use std::net::TcpStream;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Receiver, Sender, RecvError};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::thread_client::ThreadClient;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use crate::wait_close_input;
use termion::event::Key::Delete;


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

    pub fn delete_offline_clients(&self) {
       let mut clients_unlocked = self.clients.lock().unwrap();
        for i in 0..clients_unlocked.len() {
            if !clients_unlocked[i].is_alive() {
                clients_unlocked[i].join();
            }
        }
        clients_unlocked.retain( |x| x.is_alive() );
    }

    pub fn update_clients(&self) {
        self.server_running.store(true, Ordering::Relaxed);
        /*This iterator will block whenever next is called,
        waiting for a new message, and None will be returned
        when the corresponding channel has hung up */
        //for message in self.messages.lock().unwrap().iter() { //blocking

        for message in self.messages.lock().unwrap().iter() {
            if !self.server_running.load(Ordering::Relaxed) { break; }
            for client in self.clients.lock().unwrap().iter_mut() {
                client.send_message(&message);
            }
        }
    }

    pub fn close(&self) {
        for client in self.clients.lock().unwrap().iter_mut() {
            client.close();
        }

        self.server_running.store(false, Ordering::Relaxed);
        let good_bye_message = String::from("\n");
        self.sender.lock().unwrap().send(good_bye_message).unwrap();
    }
}