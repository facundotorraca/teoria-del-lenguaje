use std::io;
use crate::server::Server;

mod server;
mod chat_room;
mod thread_client;

fn main() -> io::Result<()> {
    println!("/*-----------------------------------------------------*/");
    println!("/*             FaceRust Messenger Server               */");
    println!("/*-----------------------------------------------------*/");

    /* "?" -> "error propagation" */
    println!("/*------------------Introduce a PORT-------------------*/");
    let mut port= String::new();
    io::stdin().read_line(&mut port)?;
    port = port.replace("\n", "");
    println!("/*-----------------------------------------------------*/");

    println!("/*------------------Introduce a HOST-------------------*/");
    let mut host = String::new();
    io::stdin().read_line(&mut host)?;
    host = host.replace("\n", "");
    println!("/*-----------------------------------------------------*/");

    let mut server = Server::new(&host, &port);
    server.start();

    return Ok(());
}



