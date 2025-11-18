use crate::ipgrabber;
use std::io::{BufReader, prelude::*, Read, Write};
use tokio::net::TcpListener;
use std::io;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;
use tokio::sync::broadcast;
use tokio::io::AsyncBufReadExt;
pub async fn server(ip: String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(&ip).await?;
    let (mut tx, _rx) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);
    
    loop {
    match listener.accept().await{

        let tx_cloned = Arc::clone(&tx);
        let mut rx = tx_cloned.subscribe();
        match listener.accept().await {
        Ok((mut stream, _)) => {
            tokio::spawn(async move {
                println!("Connection recieved.");
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
                result = result.unwrap().to_string();
                tx_clone.send(result).unwrap();
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
