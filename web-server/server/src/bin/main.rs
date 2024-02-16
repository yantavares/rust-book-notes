use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    // First of all we need to define the TCP listener
    let address = "127.0.0.1:7878";

    let listener = TcpListener::bind(&address).unwrap();

    let pool = ThreadPool::new(4); // Creating a thread pool with a fixed number of threads

    // We need to iterate over the incoming connections
    // take(2) ensures that the listener will only handle two connections
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // We could create a new thread for each connection,
        // but this is not a good idea because it could lead to a DoS attack

        pool.execute(|| {
            // Establishing the connection
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // We need to define the buffer to store the data
    let mut buffer = [0; 512];

    // We need to read the data from the stream
    stream.read(&mut buffer).unwrap();

    // We need to define the request
    // b"GET / HTTP/1.1\r\n" is a byte string
    let get = b"GET / HTTP/1.1\r\n";
    // This requests will sleep for 5 seconds to test the thread pool
    // This simmulates a slow request
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // We need to check if the request is a GET request
    let (status, filename) = if buffer.starts_with(get) {
        // If the request is a GET request we need to call the response function
        ("200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        // If it is a sleep request we need to sleep for 5 seconds
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "index.html")
    } else {
        // If the request is not a GET request we need to call the 404 function
        ("404 NOT FOUND", "404.html")
    };

    // Reading contents of the html file
    let contents = fs::read_to_string(filename).unwrap();

    // We need to define the response
    // The format! macro is similar to println! but instead of printing the output to the console
    // it returns a string with the formatted text
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        // contents-length is the length of the contents in bytes
        status,
        contents.len(),
        contents
    );

    // We need to write the response to the stream
    stream.write(response.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing until
    // all the bytes are written to the connection
    stream.flush().unwrap();
}
