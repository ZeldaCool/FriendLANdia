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
use tokio::net::TcpStream;
pub async fn server(ip: String) -> Result<(), Box<dyn Error>> {
    //use rwlock instead
    let clientip = Arc::new(RwLock::new(Vec::new()));
    let mut counter = 0;
    let listener = TcpListener::bind(&ip).await?;
   

    loop {
        let cloned_writer =  Arc::clone(&clientip);
        let cloned_reader =  Arc::clone(&clientip);
        match listener.accept().await{
        Ok((mut stream, addr)) => {
            tokio::spawn(async move {
                //Retrieve write lock here
                let mut vec = cloned_writer.write().await;
                vec.push(addr);
                drop(vec);
                println!("Connection recieved from : {}", addr);
                let mut buffer = [0; 512];
                while let Ok(n) = stream.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                let mut result = task::spawn_blocking(|| ->  String {
                println!("Enter message...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                input.trim().to_string()
                }).await;
                //See if you need to drop lock here and grab it later
                //stream.write_all(result.unwrap().as_bytes()).await;
                //Grab read lock here
                //Figure out how to iterate here
                //let mut vec_read = cloned_reader.read().await;
                /*for i in vec_read{
                    //This will open a tcp stream to each client, client listener task will bind a tcpserver with their ip and a specific port
                    //todo!();
                    //counter = counter+1;
                }*/
                }
                });
            },

        Err(e) => println!("couldn't get client: {:?}", e),
        }   
    }
    Ok(())
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
