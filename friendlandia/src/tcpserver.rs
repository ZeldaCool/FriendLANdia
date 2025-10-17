use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::io;
pub fn tcpserver(ip: String) -> String {
    //Add async thread handler for multiple messages
    let listener = TcpListener::bind(ip).unwrap();
    let hi = "Server Shutting Down...";
    for stream in listener.incoming() {
        println!("Message incoming, accept it? Y/N");
        let mut responsehere = String::new();
        let useresponse = io::stdin().read_line(&mut responsehere).expect("Failure");
        if responsehere.trim() == "Y"{
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let message: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println!("{:?}", message);
            let clientip = message;
        }
        else{
            todo!();
        }
        
        
    }
    hi.to_string()
}
pub fn moderation(){
    todo!();
    //Add moderation and editing messages before forwarding
}