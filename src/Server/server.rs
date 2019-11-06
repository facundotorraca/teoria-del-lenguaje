use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::sync::{Mutex, Arc};
use crate::chat_room::ChatRoom;
use std::sync::atomic::{AtomicBool, Ordering};
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

const MAX_CLIENTS: usize = 32;

fn wait_clients(listener: Arc<Mutex<TcpListener>>, chat_room: Arc<ChatRoom>) {
    let listener_shared = listener.lock().unwrap();
    //listener.set_nonblocking(true).expect("Cannot set non-blocking");

    println!("/*-----------------WAINTING_NEW_CLIENTS----------------*/");
    for stream in listener_shared.incoming() {
        chat_room.delete_offline_clients();
        let number_of_clients = chat_room.add_client(stream.unwrap());
        println!("New Client accepted: {} clients connected", number_of_clients);
    }
}

pub struct Server {
    running: AtomicBool,
    chat_room: Arc<ChatRoom>,
    listener: Arc<Mutex<TcpListener>>,
}

impl Server {
    pub fn new(host: &str, port: &str) ->  Server{
        let addr = host.to_owned() + ":" + port;

        let listener = TcpListener::bind(&addr).unwrap();

        println!("/*----------------------BIND SUCCESS-------------------*/");
        println!("ADDR: {}", addr);            /* here chat_room is unlocked */
        println!("/*-----------------------------------------------------*/");

        Server{listener: Arc::new(Mutex::new(listener)),
               running: AtomicBool::new(false),
               chat_room: Arc::new(ChatRoom::new(MAX_CLIENTS))}
    }

    pub fn start(&self) {
        let listener_clone = self.listener.clone();
        let chat_clone = self.chat_room.clone();

        let handler =  thread::spawn(move || { wait_clients(listener_clone, chat_clone); });

        self.running.store(true, Ordering::Relaxed);

        while self.running.load(Ordering::Relaxed) {
            self.chat_room.update_clients();
        }

        println!("METILDE");
        handler.join();
        println!("MENTIRA");
    }

    pub fn close(&self) {
        self.chat_room.close();
        self.running.store(false, Ordering::Relaxed);
    }
}
