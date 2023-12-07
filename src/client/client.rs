use std::io::Write;

use tokio::io::{AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // connecting to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to server!");

    // read user input and send it to the server
    loop {
        // prompt user for input
        print!("Enter a message: ");
        let _ = std::io::stdout().flush(); // Ensure the prompt is displayed
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // input to server
        stream.write_all(input.as_bytes()).await.expect("Failed to write to socket");
    }
}

