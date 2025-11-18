use std::process::{Command, Stdio};
use std::io;
mod ipgrabber;
mod tcpserver;
mod tcpclient;
//TODO: Tcpserver handles one client at a time, stores messages with timestamps into a vec and forwards it to other client(s)
//TODO: Use thread::spawn for tcpserver
//TODO: Moderation messages, message ids
//TODO: Text writer w/messages, make a reader for it too
//Decide about tokio
fn main(){
    println!("Server or client? server/client");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    if a.trim() == "server"{
        tcpserver::main();
    } else{
        println!("Enter server's ip w/ :55000 after it");
        let mut a = String::new();
        let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
        tcpclient::client(a);
    }
   
}