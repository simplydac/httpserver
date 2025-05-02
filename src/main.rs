use std::net::TcpListener;
use std::io::{Read, Write};

fn get_host(s: &str) -> Option<&str> {
    for line in s.lines() {
        if line.starts_with("Host:") {
            let host = line.split_whitespace().nth(1)?;
            return Some(host);
        }
    }
    None
}

fn handle_bla() -> String {
    "Blabla!".to_string()
}

fn handle_ble() -> String {
    "Hello, world from ble!".to_string()
}

fn main() {
    // Bind the server to the address
    let addr = "localhost:3000"; // "{interface}:{port}"
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listening on http://{addr}");
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
        // println!("The bytes:\n{:?}", &buffer[..n]);
        println!("The string:\n{}", s);
        let host = get_host(&s);
        let contents = match host {
            Some("bla.localhost:3000") => handle_bla(),
            Some("ble.localhost:3000") => handle_ble(),
            _ => "Not found".to_string(),
        };
        let response = format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}
