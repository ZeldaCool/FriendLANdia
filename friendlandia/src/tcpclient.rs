use std::{
    fs,
    io::{BufReader, prelude::*, Read, Write},
    net::TcpStream,
};
use std::io;
use tokio::sync::broadcast;
use tokio::task;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio::sync::RwLock;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::net::TcpStream;
//Utilize a blocking task for client listener/sender, and a non-blocking task for the broadcast listener


pub fn client(serverip: String) ->  Result<(), Box<dyn Error>> {
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
    let mut bufferclient = [0; 512];
    let mut bytes_read = stream.read(&mut bufferclient).unwrap();
    println!("Response: {}", String::from_utf8_lossy(&bufferclient[..bytes_read]));
    } else{
        break;
    }    
    }
    Ok(())
}
/*pub async fn client(ip: String) -> Result<(), Box<dyn Error>>{
    let serverip = ip.trim().to_string();
    let mut stream = TcpStream::connect(&serverip).await?;
    let mut buffer = [0; 512];
    //Stream Task
    tokio::spawn(async move {
        loop{
            let mut result = task::spawn_blocking(|| ->  String {
            println!("Enter message...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input);
            input.trim().to_string()
            }).await;
            stream.write_all(result.unwrap().as_bytes()).await;
            match stream.read(&mut buffer).await{
                Ok(n) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
    }
    });
    /*Broadcast Listener
    tokio::spawn({
        //Add verification, if broadcast is recieved and the user tag isn't included ignore and don't display
    });*/
    Ok(())
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter server's ip w/ :55000 after it");
    let mut a = String::new();
    let useresponsea = io::stdin().read_line(&mut a).expect("Failure");
    client(a).await?;
    Ok(())
}*/