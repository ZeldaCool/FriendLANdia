use std::process::{Command, Stdio};
use std::io;
mod ipgrabber;
mod tcpserver;
mod tcpclient;
//TODO: Tcpserver handles one client at a time, stores messages with timestamps into a vec and forwards it to other client
//TODO: Use thread::spawn for tcpserver
//TODO: Moderation messages, message ids
//TODO: Text writer w/messages, make a reader for it too
fn main(){
    println!("Server or client? server/client");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    /*let mut x = ipgrabber::get_ip();
    let mut z = x.clone();
    println!("{:?}", z);
    z.push(':');
    z.push('7');
    z.push('8');
    z.push('7');
    z.push('8');
    let s = z.into_iter().collect::<String>();
    let x = x.into_iter().collect::<String>();
    println!("{}", s);*/
    if a.trim() == "server"{
        tcpserver::server_conn();
    } else{
        tcpclient::client();
    }
   
}