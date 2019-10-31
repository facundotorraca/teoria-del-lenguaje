use std::thread;
use std::net::TcpListener;
use std::sync::{Mutex, Arc};
use crate::chat_room::ChatRoom;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

const MAX_CLIENTS: usize = 32;

fn wait_clients(listener: Arc<Mutex<TcpListener>>, chat_room: Arc<ChatRoom>) {
    let listener_shared = listener.lock().unwrap();

    println!("/*-----------------WAINTING_NEW_CLIENTS----------------*/");
    for stream in listener_shared.incoming() {
        println!("adding client");
        let number_of_clients = chat_room.add_client(stream.unwrap());
        println!("New Client accepted: {} clients connected", number_of_clients);
    }
}

pub struct Server {
    running: AtomicBool,
    listener: Arc<Mutex<TcpListener>>,
}

impl Server {
    pub fn new(host: &str, port: &str) ->  Server{
        let addr = host.to_owned() + ":" + port;

        let listener = TcpListener::bind(&addr).unwrap();

        println!("/*----------------------BIND SUCCESS-------------------*/");
        println!("ADDR: {}", addr);
        println!("/*-----------------------------------------------------*/");

        Server{listener: Arc::new(Mutex::new(listener)),
               running: AtomicBool::new(false)}
    }

    pub fn start(&mut self) {
        let listener_clone = self.listener.clone();
        let chat_room = Arc::new(ChatRoom::new(MAX_CLIENTS));
        let chat_clone = chat_room.clone();

        let handler =  thread::spawn(move || { wait_clients(listener_clone, chat_clone); });

        *self.running.get_mut() = true; //Atomic bool running -> true

        while *self.running.get_mut() {
            chat_room.update_clients();
            /* here chat_room is unlocked */
        }

        handler.join();
    }
}
