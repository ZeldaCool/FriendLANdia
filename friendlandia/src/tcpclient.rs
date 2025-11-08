use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::TcpStream,
};
//use tokio::net::TcpStream;
use std::io;
use tokio::sync::broadcast;
use tokio::task;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio::sync::RwLock;
use tokio::sync::Mutex;

//work on async client
pub fn client(serverip: String) -> std::io::Result<()> {
    //Figure out how to use the serverip to connect tcpstream
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    loop {
    println!("Send a message? Y/N");
    let mut a = String::new();
    let useresponse = io::stdin().read_line(&mut a).expect("Failure");
    if a.trim() == "Y"{
    println!("Enter message...");
    let mut aa = String::new();
    let useresponsee = io::stdin().read_line(&mut aa).expect("Failure");
    let aa = aa.as_bytes();
    stream.write_all(aa).unwrap();
    let mut bufferclient = [0; 512];
    let mut bytes_read = stream.read(&mut bufferclient).unwrap();
    println!("Response: {}", String::from_utf8_lossy(&bufferclient[..bytes_read]));
    } else{
        break;
    }    
    }
    Ok(())
}
/*#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    //TCPStream Task
    tokio::spawn(async move {

    });
    //Message Broadcast Listener
    tokio::spawn(async move {

    });
}*/