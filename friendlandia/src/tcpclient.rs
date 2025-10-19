use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::{TcpListener, TcpStream},
};
//create echo implementation
use std::io;
pub fn client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    stream.write_all(b"Hello, world!").unwrap();
    let mut bufferclient = [0; 512];
    let mut bytes_read = stream.read(&mut bufferclient).unwrap();
    println!("Message: {}", String::from_utf8_lossy(&bufferclient[..bytes_read]));    
    Ok(())
} 