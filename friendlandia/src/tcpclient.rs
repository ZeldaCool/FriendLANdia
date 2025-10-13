use std::{
    io::{BufReader, prelude::*, Read, Write},
    net::{TcpListener, TcpStream},
};
use std::io;
pub fn tcpclient(ip: String) -> std::io::Result<()> {
    let sender = TcpStream::connect(ip)?;
    sender.write(b"Hello, world!");
    let mut bytereader = [0; 128];
    let mut response = stream.read(&mut bytereader)?;
    println!("Response: {}", String::from_utf8_lossy(&bytereader[..response]));
    Ok(())
}