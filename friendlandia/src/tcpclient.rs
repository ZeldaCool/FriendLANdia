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
use std::time::Duration;
use std::thread::sleep;
use std::sync::mpsc::{channel, Sender};
use std::time::Instant;
//use tokio::net::TcpStream;
//Utilize a blocking task for client listener/sender, and a non-blocking task for the broadcast listener


pub fn client(serverip: String) ->  Result<(), Box<dyn Error>> {
    let serverip = serverip.trim().to_string();
    let mut stream = TcpStream::connect(&serverip)?;
    let mut buffer = [0; 512];
    let mut read_stream = stream.try_clone()?;
    let (sig_tx, sig_rx) = channel::<()>();
    let mut first_message_local = true;
    let mut counter = 0;
    thread::spawn(move || {
        let mut buffer = [0u8; 512];
        let mut first_message = first_message_local;
        let mut stream_read = read_stream;
        loop{
            let listen_duration_first = Duration::from_secs(2);
            let listen_duration_otherwise = Duration::from_secs(20);
            let start = Instant::now();
            let mut counter = 0;
            if first_message{
            while start.elapsed() < listen_duration_first{
            match stream_read.read(&mut buffer){
                Ok(n) =>{
                    if n == 0{
                        break;
                    }
                    println!("\n{}", String::from_utf8_lossy(&buffer[..n]));
                    if String::from_utf8_lossy(&buffer[..n]) == "Hello, client!"{
                        let _ = sig_tx.send(());
                        break;
                    }
                }
                Err(e) =>{
                    println!("Error: {}", e);
                    break;
                }
            }
        }
        let _ = sig_tx.send(());
        first_message = false;
        } else{
            while start.elapsed() < listen_duration_otherwise{
                if counter >= 2{
                    let _ = sig_tx.send(());
                    break;
                }
                match stream_read.read(&mut buffer){
                Ok(n) =>{
                    if n == 0{
                        break;
                    }
                    println!("\n{}", String::from_utf8_lossy(&buffer[..n]));
                    counter += 1;
                }
                Err(e) =>{
                    println!("Error: {}", e);
                    break;
                }
                }
            }
        }
        
        }
    });
    loop{
    match sig_rx.recv_timeout(std::time::Duration::from_secs(20)) {
        Ok(_) => {
        }
        Err(_) => {
        }
    }
    while sig_rx.try_recv().is_ok() {}

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
    let awaiting_response = true;
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