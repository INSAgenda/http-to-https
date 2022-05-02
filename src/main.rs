#![allow(clippy::all)]

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    #[cfg(not(debug_assertions))]
    let listener = TcpListener::bind("[::]:80").unwrap();
    #[cfg(debug_assertions)]
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut counter = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        counter += 1;

        handle_connection(stream);

        if (counter <= 10) || (counter <= 100 && counter % 10 == 0) || (counter <= 1000 && counter % 100 == 0) || (counter % 1000 == 0) {
            println!("Hits: {}", counter);
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Vec::with_capacity(1024);
    unsafe { buffer.set_len(1024) };

    stream.read(&mut buffer).unwrap();

    if let Some(Ok(first_line)) = buffer.lines().next() {
        if let Some(path) = first_line.split(' ').nth(1) {
            let response = format!("HTTP/1.1 301 Moved Permanently\r\nLocation: https://insagenda.fr{path}\r\n\r\n");

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    }

    eprintln!("Could not parse http request");
    let response = "HTTP/1.1 301 Moved Permanently\r\nLocation: https://insagenda.fr\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
