use std::process::{Command, Stdio};
use std::io;
mod ipgrabber;
mod tcpserver;
mod tcpclient;
fn main(){
    println!("Server or client? server/client");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    let mut x = ipgrabber::get_ip();
    let mut z = x.clone();
    println!("{:?}", z);
    z.push(':');
    z.push('7');
    z.push('8');
    z.push('7');
    z.push('8');
    let s = z.into_iter().collect::<String>();
    let x = x.into_iter().collect::<String>();
    println!("{}", s);
    if a.trim() == "server"{
    tcpserver::tcpserver(s.to_string());
    } else{
        println!("Enter the server's ip here...");
        let mut b = String::new();
        let useresponseaa = io::stdin().read_line(&mut b).expect("Failure");
        tcpclient::tcpclient(b.to_string(), x.to_string());
    }
   
}