use crate::ipgrabber;
use std::collections::HashMap;
use std::io::{BufReader, prelude::*, Read, Write};
use tokio::net::TcpListener;
use std::io;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use std::sync::Mutex;
use std::sync::Arc;
use tokio::task;
use tokio::sync::broadcast;
use tokio::io::AsyncBufReadExt;
use std::time::SystemTime;
use tokio::net::TcpStream;
use std::thread;
pub async fn server(ip: String) -> Result<(), Box<dyn Error>> {
    //use rwlock instead
    let (tx, mut rx) = mpsc::channel::<String>(10);
    let clientip = Arc::new(RwLock::new(Vec::new()));
    let listener = TcpListener::bind(&ip).await?;
    let mut first_message = true;
    let mut moderate = "X";
    let mut result = "X";
    loop {
        let mut client_id = 0;
        let cloned_writer =  Arc::clone(&clientip);
        let cloned_reader =  Arc::clone(&clientip);
        match listener.accept().await{
        Ok((mut stream, addr)) => {
            tokio::spawn(async move {
                //Retrieve write lock here
                let mut vec = cloned_writer.write().await;
                vec.push(addr.to_string());
                drop(vec);
                println!("Connection recieved from : {}", addr);
                let mut buffer = [0; 512];
                while let Ok(n) = stream.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                println!("Moderate message? Y/N");
                let mut moderate = input_reader().await;
                println!("Enter message...");
                let mut result = input_reader().await;
                //See if you need to drop lock here and grab it later
                //Grab read lock here
                //Figure out how to iterate here

                //Try using blocking_read in a spawn::blocking for this whole section, place ips in a vec to use it outside of task, use that as the client list? If that doesn't work, post to stackoverflow
                let mut vec_read = cloned_reader.read().await;
                
                for i in vec_read.iter(){
                    let ip = i.clone();
                    let mut ip_stuff = task::spawn_blocking(move|| -> String{
                        println!("Ip: {}", ip);
                        let new_ip = ipgrabber::port_converter(ip);
                        println!("Modified IP: {}", new_ip);
                        new_ip
                    }).await;
                    let ip_stuff = ip_stuff.unwrap().to_string();
                    let mut client_broadcast = TcpStream::connect(&ip_stuff).await.expect("connect failed");
                    let mut sending = String::from_utf8_lossy(&buffer[..n]).to_string();
                    client_broadcast.write_all(client_id.to_string().as_bytes()).await;
                    client_id = client_id+1;
                    if moderate == "Y"{
                    let timestamp = SystemTime::now();
                    let moderated_formatted = format!("ID: {} MESSAGE: MODERATED TIMESTAMP: {:?}", client_id, timestamp);
                    client_broadcast.write_all(moderated_formatted.as_bytes()).await;
                    } else{
                    let timestamp = SystemTime::now();
                    let client_formatted = format!("ID: {} MESSAGE: {} TIMESTAMP: {:?}", client_id, sending, timestamp);
                    client_broadcast.write_all(client_formatted.as_bytes()).await;
                    }
                    let timestamp = SystemTime::now();
                    let server_formatted = format!("ID: SERVER MESSAGE: {} TIMESTAMP: {:?}", client_id, result);
                    client_broadcast.write_all(server_formatted.as_bytes()).await;
                }
                }
                });
            },

        Err(e) => println!("couldn't get client: {:?}", e),
        }   
    }
    Ok(())
}

pub async fn input_reader() -> String{
    let mut moderate = task::spawn_blocking(|| ->  String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        input.trim().to_string()
    }).await;
    moderate.unwrap().to_string()
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {
    let mut z = ipgrabber::get_ip();
    z.push(':');
    z.push('5');
    z.push('5');
    z.push('0');
    z.push('0');
    z.push('0');
    let z = z.into_iter().collect::<String>();
    server(z).await;
    Ok(())
}
