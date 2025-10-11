use std::process::{Command, Stdio};
use std::io;
mod ipgrabber;
mod tcpserver;
fn main(){
    let mut x = ipgrabber::get_ip();
    println!("{:?}", x);
    x.push(':');
    x.push('7');
    x.push('8');
    x.push('7');
    x.push('8');
    let s = x.into_iter().collect::<String>();
    println!("{}", s);
    tcpserver(s);
   
}