use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// We need mut for stream, even though we are just gonna read it, because reading
// a stream can mutate its internal value. If more data is to be read than what
// can be allocated to the buffer, the fitting data will be read, and the
// remaining data will keep in the variable.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    /*  The HTTP response should look something like this:
    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    CRLF
    body
    */
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // The same stream variable is also used to write the response
    stream.write(response.as_bytes()).unwrap();
    // Flush asures the writing is successfully completed before dropping the connection
    stream.flush().unwrap();
}
