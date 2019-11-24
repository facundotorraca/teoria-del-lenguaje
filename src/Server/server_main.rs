use std::{io, thread, env};
use crate::server::Server;
use std::sync::{Arc, Mutex};

mod server;
mod chat_room;
mod thread_client;

fn wait_close_input() {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer);
    while input_buffer != "q\n".to_string() {
        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer);
    }
}

fn main() -> io::Result<()> {
    println!("/*-----------------------------------------------------*/");
    println!("/*             FaceRust Messenger Server               */");
    println!("/*-----------------------------------------------------*/");

    let host = "localhost";

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Port not specified.");
        return Ok(());
    }

    let mut server = Arc::new(Server::new(&host, &args[1]));
    let mut server_clone = server.clone();

    let handler = thread::spawn(move || { server_clone.start();});

    wait_close_input();

    server.close();
    handler.join();
    return Ok(());
}



