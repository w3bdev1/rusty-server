use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Print request
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    // Send response
    let res = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
