use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// We need mut for stream, even though we are just gonna read it, because reading
// a stream can mutate its internal value. If more data is to be read than what
// can be allocated to the buffer, the possible data will be read, and the
// remaining data will keep in the variable.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4];

    stream.read(&mut buffer).unwrap();

    println!("First 4 bytes: {}", String::from_utf8_lossy(&buffer[..]));

    let mut remaining_buffer = [0; 1024];
    stream.read(&mut remaining_buffer).unwrap();

    println!(
        "Remaining bytes: {}",
        String::from_utf8_lossy(&remaining_buffer[..])
    );
}
