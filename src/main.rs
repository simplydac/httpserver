use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // Bind the server to the address
    let addr = "192.168.1.21:3000"; // "{interface}:{port}"
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listening on {addr}");
    // Accept incoming connections
    let mut streams = listener.incoming();
    // Loop through the incoming connections
    loop {
        let mut stream = streams.next().unwrap().unwrap();
        dbg!(&stream);
        let mut buffer = [0; 5555];
        // read up to 10 bytes, put in buffer and return the number of bytes read
        let n = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer[..n]);
        println!("The bytes:\n{:?}", &buffer[..n]);
        println!("The string:\n{}", s);
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, world!\r\n").unwrap();
    }
}
