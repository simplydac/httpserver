use std::net::TcpListener;
use std::io::Read;

fn main() {
    // Bind the server to the address
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server listening on http://localhost:3000");
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
    }
    
}
