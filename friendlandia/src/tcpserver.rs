use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::io;
//Add username functionality
pub fn tcpserver(ip: String) -> String {
    //Add async thread handler for multiple messages
    //Add encryption as well
    //Add message number functionality and display for easy refrence
    //Implement moderation notices, distribute moderated messages if necessary otherwise distribute it unedited
    //Timestamps
    //Create a tcpstream spawn function to handle each seperate connection, rust only handles one at a time
    let listener = TcpListener::bind(ip).unwrap();
    let hi = "Server Shutting Down...";
    for stream in listener.incoming() {
        println!("Message incoming, accept it? Y/N");
        let mut responsehere = String::new();
        let useresponse = io::stdin().read_line(&mut responsehere).expect("Failure");
        if responsehere.trim() == "Y"{
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let message: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println!("{:?}", message);
            let clientip = message;
            let mut response = "hehehehehe!";
            stream.write_all(response.as_bytes()).unwrap();

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