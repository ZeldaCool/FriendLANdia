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
    let message_logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let client_counter = Arc::new(Mutex::new(0u32));    
    let client_amount = Arc::new(Mutex::new(0u32));
    loop {
        let client_counter_clone = Arc::clone(&client_counter);
        let clients_clone = Arc::clone(&clients);
        let client_ids_clone = Arc::clone(&client_ids);
        let client_amount_clone = Arc::clone(&client_amount);
        let message_logs_clone = Arc::clone(&message_logs);
        let mut client_id = 0;
        match listener.accept().await{
        Ok((mut stream, addr)) => {
            tokio::spawn(async move {
                let mut locked_counter = client_amount_clone.lock().await;
                *locked_counter += 1;
                if *locked_counter == 1{
                    stream.write_all("Hello, client!".as_bytes()).await;
                } else{
                    let locked_logs = message_logs_clone.lock().await;                    
                    for msg in locked_logs.iter(){
                        stream.write_all(msg.as_bytes()).await;
                    }
                }
                drop(locked_counter);
                let stream = Arc::new(Mutex::new(stream));
                let mut counter = client_counter_clone.lock().await;
                *counter += 1;
                let assigned_id = *counter;
                drop(counter);
                let key = Arc::as_ptr(&stream) as usize;
                let mut map = client_ids_clone.lock().await;
                map.insert(key, assigned_id);
                drop(map);
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
                println!("Received from {}: {}", addr, String::from_utf8_lossy(&buffer[..n]));
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
                let key = Arc::as_ptr(&stream) as usize;
                let map = client_ids_clone.lock().await;
                let client_id = map[&key];
                drop(map);
                for i in vec_read.iter(){
                    let mut locked_client = i.lock().await;
                    let sending = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if moderate == "Y"{      
                    let moderated_formatted = format!("CLIENT {} \nMESSAGE: MODERATED BY SERVER\n", client_id);
                    locked_client.write_all(moderated_formatted.as_bytes()).await;
                    locked_client.flush().await;
                    let mut locked_logs = message_logs_clone.lock().await;                    
                    locked_logs.push(moderated_formatted);
                    drop(locked_logs);
                    } else{                                            
                    let client_formatted = format!("CLIENT {} \nMESSAGE: {}\n", client_id, sending);
                    locked_client.write_all(client_formatted.as_bytes()).await;
                    locked_client.flush().await;
                    let mut locked_logs = message_logs_clone.lock().await;                    
                    locked_logs.push(client_formatted);
                    drop(locked_logs);
                    }
                    let server_formatted = format!("SERVER \nMESSAGE: {}\n", result);
                    locked_client.write_all(server_formatted.as_bytes()).await;
                    locked_client.flush().await;
                    let mut locked_logs = message_logs_clone.lock().await;                    
                    locked_logs.push(server_formatted);
                    drop(locked_logs);
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
