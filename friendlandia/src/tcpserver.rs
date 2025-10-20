use tokio::net::TcpListener;
use std::io;
use std::error::Error;

pub async fn handle_connection<T>(socket: T){
    todo!();
}
#[tokio::main]
pub async fn main() -> io::Result<()>  {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        match listener.accept().await {
        Ok((socket, _)) => {
            println!("Connection recieved.");
            tokio::spawn(async move {
            handle_connection(socket).await
        });
        },
        Err(e) => println!("couldn't get client: {:?}", e),
        }   
        
    }
}