## How to run the server:
```bash
cd server
cargo run
```

The server will be listening on `127.0.0.1:7878`.

The server is set to automatically shutdown after 2 requests for demonstration purposes.

# Chapter 20 - Creating a Multithreaded Web Server in Rust

- TCP is a connection-oriented protocol, which means that it establishes a connection between the client and the server before sending any data. It is reliable, in-order, and error-checked. It is used for applications that require high reliability and low latency, such as web servers, email servers, and file transfer.

- To implement a multithreaded web server, we will use the `std::net` module to create a TCP listener. We will then use the `std::thread` module to create a new thread for each incoming connection. We will also use the `std::fs` module to read the contents of the requested file and send it back to the client.

- Multi-threading is useful for web servers because it allows the server to handle multiple requests at the same time. This can improve the performance of the server and reduce the response time for clients.

We moved `main.rs` inside `bin` directory and created a new `lib.rs` file in the `src` directory in order to separate the server logic from the main function.
