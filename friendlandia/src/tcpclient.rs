use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
};
use std::net::TcpStream;
use std::io;
use tokio::sync::broadcast;
use tokio::task;
use std::thread;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio::sync::RwLock;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::ipgrabber;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
//use tokio::net::TcpStream;
//Utilize a blocking task for client listener/sender, and a non-blocking task for the broadcast listener


pub fn client(tx_to_broad: mpsc::Sender<String>, mut rx_from_broad: tokio::sync::mpsc::Receiver<Vec<String>>, serverip: String) ->  Result<(), Box<dyn Error>> {
    let mut message_logs: Vec<String> = vec![];
    let serverip = serverip.trim().to_string();
    let mut stream = TcpStream::connect(&serverip)?;
    loop{
    println!("Send a message? Y/N");
    let mut a = String::new();
    let useresponse = io::stdin().read_line(&mut a).expect("Failure");
    if a.trim() == "Y"{
    println!("Enter message...");
    let mut aa = String::new();
    let useresponsee = io::stdin().read_line(&mut aa).expect("Failure");
    let aa = aa.as_bytes();
    stream.write_all(aa).unwrap();
    tx_to_broad.blocking_send("OK".to_string());
    if let Some(vec_messages) = rx_from_broad.blocking_recv() {
                for msg in vec_messages {
                    println!("{}", msg);
                }
            }
    } else{
        break;
    }    
    }
    Ok(())
}
pub async fn broadcaster(mut rx_from_client: mpsc::Receiver<String>, tx_to_client: tokio::sync::mpsc::Sender<Vec<String>>, ip: String) -> Result<(), Box<dyn Error>>{
    //Append the new messages to the message log, clear screen w/ this:     print!("{}[2J", 27 as char);
    let mut buffer = [0; 512];
    let mut first_message = true;
    let listener = TcpListener::bind(&ip).await?;
    let mut client_id = "undefined".to_string();
    while let Some(_) = rx_from_client.recv().await {
        let mut messages = vec![];
        match listener.accept().await{
                Ok((mut stream, addr)) => {
                    if first_message{
                        let mut i = 0;
                        while i <= 2{
                            match stream.read(&mut buffer).await{
                            Ok(n) => {
                                if i == 0{
                                    let client_id = String::from_utf8_lossy(&buffer[..n]);
                                    messages.push(client_id.to_string());
                                    let i = i+1;

                                } else{
                                let mut message = String::from_utf8_lossy(&buffer[..n]);
                                messages.push(message.to_string());
                                let i = i+1;
                                }
                            },
                            Err(e) => println!("Error: {}", e),
                            };
                        }

                    } else{
                    let mut i = 1;
                    while i <= 2{
                        match stream.read(&mut buffer).await{
                            Ok(n) => {
                                let mut message = String::from_utf8_lossy(&buffer[..n]);
                                messages.push(message.to_string());
                            },
                            Err(e) => println!("Error: {}", e),
                        };
                        i = i+1;
                    }
                    }
                    let first_message = false;
                },
                Err(e) => println!("Error: {}", e),
        }
        tx_to_client.send(messages).await.unwrap();
    }
    Ok(())
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let (tx_client_to_broad, rx_client_to_broad) = mpsc::channel(32);
    let (tx_broad_to_client, rx_broad_to_client) = mpsc::channel(32);    
    println!("Enter server's ip w/ :55000 after it");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    let mut usersip = ipgrabber::get_ip();
    usersip.push('5');
    usersip.push('5');
    usersip.push('1');
    usersip.push('1');
    usersip.push('1');
    let usersip = usersip.into_iter().collect::<String>();
    let a = a.trim().to_string();
    let client = task::spawn_blocking(move || {
        client(tx_client_to_broad, rx_broad_to_client, a).unwrap();
    });
    let broadcast = tokio::spawn(async move{
        broadcaster(rx_client_to_broad, tx_broad_to_client, usersip).await.unwrap();
    });
    tokio::join!(client, broadcast);
    Ok(())
}