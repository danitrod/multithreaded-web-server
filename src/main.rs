use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let port = std::env::var("PORT").unwrap_or("7878".into());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    println!("Listening on port {}", port);

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

    // Validate that the request is a GET /
    const GET_INDEX: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(GET_INDEX) {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else {
        // Return a 404 if not
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    /* The HTTP response should look something like this
    (defined here: https://www.w3.org/Protocols/rfc2616/rfc2616-sec6.html):

    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    CRLF
    body
    */
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // The same stream variable is also used to write the response
    stream.write(response.as_bytes()).unwrap();
    // Flush asures the writing is successfully completed before dropping the connection
    stream.flush().unwrap();
}
