use tokio::net::TcpListener;
use std::io;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


pub async fn handle_connection<T>(stream: T){
    todo!();
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        match listener.accept().await {
        Ok((stream, _)) => {
            println!("Connection recieved.");
            tokio::spawn(async move {
            handle_connection(stream).await
        });
        },
        Err(e) => println!("couldn't get client: {:?}", e),
        }   
        
    }
    Ok(())
}