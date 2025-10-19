use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::{TcpListener, TcpStream},
};
use std::io;

pub fn client() -> std::io::Result<()> {
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