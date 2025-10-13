use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::io;
pub fn tcpserver(ip: String) -> String {
    let listener = TcpListener::bind(ip).unwrap();
    let hi = "hello, I'm cool";
    for stream in listener.incoming() {
        println!("Connection incoming, accept it? Y/N");
        let mut responsehere = String::new();
        let useresponse = io::stdin().read_line(&mut responsehere).expect("Failure");
        if useresponse.trim() == "Y"{
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let message: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println("{:?}", message);
        }
        else{
            todo!();
        }
        
        
    }
    hi.to_string()
}