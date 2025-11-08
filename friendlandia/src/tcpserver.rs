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
//Use tokio spawn_blocking task to handle user input
//Utilize tokio's mpsc to broadcast messages between tasks

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let mut buffer = [0; 512];
    loop {
        match listener.accept().await {
        Ok((mut stream, _)) => {
            println!("Connection recieved.");
            tokio::spawn(async move {
                while let Ok(n) = stream.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                let result = tokio::task::spawn_blocking(|| -> Result<String, io::Error> {
                println!("Enter message...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                println!("{:?}", input.trim().to_string());
                Ok(input.trim().to_string())
                }).await;
                println!("{:?}", result);
                //stream.write_all(b"{:?result}").await;

            }
        });
        },
        Err(e) => println!("couldn't get client: {:?}", e),
        }   
        
    }
    Ok(())
}
