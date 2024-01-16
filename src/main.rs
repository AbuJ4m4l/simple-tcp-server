use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from cilent!");
    //this is converting incoming data into UTF-8 sting
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recived Request: {}", request);
    let response = "Hello Client".as_bytes();
    stream.write(response).expect("Failed to send response");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to run server");
    println!("Server listening on 127.0.0.1:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprintln!("Failed to establish the connection: {}", err)
            }
        }
    }
}
