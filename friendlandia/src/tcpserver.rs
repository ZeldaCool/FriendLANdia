use std::net::TcpListener;

pub fn tcpserver(ip: String) -> String {
    let listener = TcpListener::bind(ip).unwrap();
    let hi = "hello, I'm cool";
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
    hi.to_string()
}