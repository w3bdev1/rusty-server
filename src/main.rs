use std::{
    fs,
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

    // Read HTML
    let html = fs::read_to_string("index.html").unwrap();

    // Send response
    let res = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
