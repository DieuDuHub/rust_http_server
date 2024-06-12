use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write(b"Hello, server!").unwrap();

    let mut buffer = [0; 5];

    stream.read(&mut buffer).unwrap();

    println!("Got response from server:{:?}", str::from_utf8(&buffer).unwrap());
}
