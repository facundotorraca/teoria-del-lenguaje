use std::io;
use crate::client::Client;
extern crate termion;

mod client;

fn main() -> io::Result<()> {
    println!("/*-----------------------------------------------------*/");
    println!("/*                FaceRust Messenger                   */");
    println!("/*-----------------------------------------------------*/");

    println!("/*------------------Introduce a PORT-------------------*/");
    let mut port = String::new();
    io::stdin().read_line(&mut port)?;
    port = port.replace("\n", "");
    println!("/*-----------------------------------------------------*/");

    println!("/*------------------Introduce a HOST-------------------*/");
    let mut host = String::new();
    io::stdin().read_line(&mut host)?;
    host = host.replace("\n", "");
    println!("/*-----------------------------------------------------*/");

    let mut client = Client::new(&host, &port);

    println!("/*---------------Introduce a USERNAME------------------*/");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    username = username.replace("\n", "");

    while !client.set_username(&username) {
        println!("Username already used, please choose another");
        username.clear();
        io::stdin().read_line(&mut username)?;
        username = username.replace("\n", "");
    }
    println!("/*-----------------------------------------------------*/");

    client.start();

    return Ok(());
}
