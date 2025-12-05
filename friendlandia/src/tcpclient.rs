use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
};
use std::net::TcpStream;
use std::io;
use tokio::sync::broadcast;
use tokio::task;
use std::thread;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio::sync::RwLock;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::ipgrabber;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tokio::time::timeout;
//use tokio::net::TcpStream;
//Utilize a blocking task for client listener/sender, and a non-blocking task for the broadcast listener


pub fn client(serverip: String) ->  Result<(), Box<dyn Error>> {
    let mut message_logs: Vec<String> = vec![];
    let serverip = serverip.trim().to_string();
    let mut stream = TcpStream::connect(&serverip)?;
    let mut buffer = [0; 512];
    let mut read_stream = stream.try_clone()?;
    thread::spawn(move || {
        let mut buffer = [0u8; 512];
        let mut stream_read = read_stream;
        loop{
            match stream_read.read(&mut buffer){
                Ok(n) =>{
                    if n == 0{
                        break;
                    }
                    println!("\n{}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(e) =>{
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    });
    loop{
    println!("\nSend a message? Y/N");
    let mut a = String::new();
    let useresponse = io::stdin().read_line(&mut a).expect("Failure");
    if a.trim() == "Y"{
    println!("Enter message...");
    let mut aa = String::new();
    let useresponsee = io::stdin().read_line(&mut aa).expect("Failure");
    let aa = aa.as_bytes();
    stream.write_all(aa).unwrap();
    stream.flush();
    }
    }
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter server's ip w/ :55000 after it");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    let a = a.trim().to_string();
    let client = client(a).unwrap();
    Ok(())
}