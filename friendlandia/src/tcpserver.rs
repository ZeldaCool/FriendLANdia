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
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;
use tokio::sync::broadcast;
use tokio::io::AsyncBufReadExt;
use std::time::SystemTime;
use tokio::net::TcpStream;
use std::thread;
pub async fn server(ip: String) -> Result<(), Box<dyn Error>> {
    let clients: Arc<tokio::sync::Mutex<Vec<Arc<tokio::sync::Mutex<TcpStream>>>>> = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let listener = TcpListener::bind(&ip).await?;
    let client_counter = Arc::new(tokio::sync::Mutex::new(0u32));
    let mut first_message = true;
    let client_ids: Arc<Mutex<HashMap<usize, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut moderate = "X";
    let mut result = "X";
    loop {
        let client_counter_clone = Arc::clone(&client_counter);
        let clients_clone = Arc::clone(&clients);
        let mut client_id = 0;
        match listener.accept().await{
        Ok((mut stream, addr)) => {
            tokio::spawn(async move {
                let stream = Arc::new(Mutex::new(stream));
                {
                    let mut locked = clients_clone.lock().await;
                    locked.push(Arc::clone(&stream));
                    drop(locked);
                }
                println!("Connection recieved from : {}", addr);
                let mut buffer = [0; 512];
                while let Ok(n) = {
                    let mut locked_stream = stream.lock().await;
                    locked_stream.read(&mut buffer).await                
                }{
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
                let vec_read = {
                    let guard = clients_clone.lock().await;
                    guard.clone()
                };
                let mut id_lock = client_counter_clone.lock().await;
                *id_lock += 1;
                let client_id = *id_lock;
                drop(id_lock);
                for i in vec_read.iter(){
                    let mut locked_client = i.lock().await;
                    let sending = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if moderate == "Y"{      
                    let moderated_formatted = format!("ID: {}| MESSAGE: MODERATED BY SERVER\n", client_id);
                    locked_client.write_all(moderated_formatted.as_bytes()).await;
                    locked_client.flush().await;
                    } else{                                            
                    let client_formatted = format!("ID: {}| MESSAGE: {}\n", client_id, sending);
                    locked_client.write_all(client_formatted.as_bytes()).await;
                    locked_client.flush().await;
                    }
                    let server_formatted = format!("ID: SERVER| MESSAGE: {}\n", result);
                    locked_client.write_all(server_formatted.as_bytes()).await;
                    locked_client.flush().await;
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
