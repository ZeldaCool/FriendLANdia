use std::process::{Command, Stdio};
use std::io;
mod ipgrabber;
mod tcpserver;
fn main(){
    println!("Are you recieving the connection? \n Y/N");
    let mut valconninfo = String::new();
    io::stdin().read_line(&mut valconninfo).expect("Failed to read");
    valconninfo = valconninfo.trim().to_string();
    if valconninfo == "Y"{
        let x = ipgrabber::get_ip();
        println!("{:?}", x);
        let s = x.into_iter().collect::<String>();
        let y = tcpserver::tcpserver();
    } else {
        todo!("Not done with this yet!");
    }
}