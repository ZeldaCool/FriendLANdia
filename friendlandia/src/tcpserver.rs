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
//
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
                let response = b"Hello, client!";
                stream.write_all(response).await.expect("Failed to write");
            }
        });
        },
        Err(e) => println!("couldn't get client: {:?}", e),
        }   
        
    }
    Ok(())
}