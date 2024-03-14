use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_connection(mut stream: TcpStream) {
    let response = "Hello from server";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connection established!");
        
        handle_connection(stream);
    }

    Ok(())
}
