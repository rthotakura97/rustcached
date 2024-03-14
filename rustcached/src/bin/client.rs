use std::io::{self, Read};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000")?;

    println!("Connected to the server!");

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("Received: {}", response);

    Ok(())
}
