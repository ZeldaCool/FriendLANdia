use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::{TcpListener, TcpStream},
};
use std::io;
pub fn tcpclient(serverip: String, userip: String) -> std::io::Result<()> {
    //TODO: For CLI Tool, implement stream ip so it'll connect
    let first_time = true;
    let mut sender = TcpStream::connect(serverip)?;
    while 1 == 1{
        println!("Send message? Y/N");
        let mut responsehere = String::new();
        let useresponse = io::stdin().read_line(&mut responsehere).expect("Failure");
        if responsehere.trim() == "Y"{
            if first_time{
                sender.write(b"{userip}");
                let mut bytereader = [0; 128];
                let mut response = sender.read(&mut bytereader)?;
                println!("Response: {}", String::from_utf8_lossy(&bytereader[..response]));
            } else{
                println!("Enter the message to send: ");
                let message_to_send = String::new();
                let response_a = io::stdin().read_line(&mut responsehere).expect("Failure");
                sender.write(b"{response_a}");
            }
            
        }
        else{
            break;
        }
    }
    Ok(())
}