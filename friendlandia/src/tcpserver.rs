use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::{TcpListener, TcpStream},
};
//create echo implementation
use std::io;
pub fn server_conn() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())

}
pub fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut bufferserver = [0; 512];
    let mut bytes_read = stream.read(&mut bufferserver).unwrap();
    println!("Message: {}", String::from_utf8_lossy(&bufferserver[..bytes_read]));    
    let mut response = stream.write_all(b"Recieved!").unwrap();
    println!("YAYAA!!!");
    Ok(())
}