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
    loop{
    let mut bufferserver = [0; 512];
    let mut bytes_read = stream.read(&mut bufferserver).unwrap();
    println!("Accept a message? Y/N");
    let mut a = String::new();
    let useresponse = io::stdin().read_line(&mut a).expect("Failure");
    if a.trim() == "Y"{
    println!("Message: {}", String::from_utf8_lossy(&bufferserver[..bytes_read]));    
    println!("Respond? Y/N");
    let mut aa = String::new();
    let useresponseee = io::stdin().read_line(&mut aa).expect("Failure");
    if aa.trim() == "Y"{
        println!("Enter response...");
        let mut aaa = String::new();
        let useresponsee = io::stdin().read_line(&mut aaa).expect("Failure");
        let aaa = aaa.as_bytes();
        stream.write_all(aaa).unwrap();
    }

    } else{
        break;
    }
    }
    Ok(())
}