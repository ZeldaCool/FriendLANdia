use std::net::TcpListener;

pub fn tcpserver(ip: String) -> String {
    let listener = TcpListener::bind("0.0.0.0").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}