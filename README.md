# Basic TCP Server in Rust 🚀

A simple Rust application that creates a TCP server, listens for incoming connections, and responds to clients.

## Description 📝

The application sets up a TCP server listening on `127.0.0.1:8080`. It handles incoming client connections, reads the data sent by the client, and sends a response back.

## How It Works 🔧

- The server listens for incoming TCP connections.
- Upon receiving a connection, it reads data from the client.
- It then sends a "Hello Client" message back to the client.
- The server can handle multiple clients simultaneously using threads.

## Usage 💻

1. Run the server program.
2. Connect to the server using a TCP client (like telnet) on `127.0.0.1:8080`.
3. Send a message to the server.
4. The server will respond with "Hello Client".

## Code Snippet 📌

```rust
use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };

fn handle_client(mut stream: TcpStream) {
    // ... [Function Implementation]
}

fn main() {
    // ... [TCP Server Setup]
}
```

## Note 🚨

- This server is a basic example and does not include error handling or security features.
- It's designed for educational purposes and should not be used as-is in production environments.

---

Happy coding! 😄
