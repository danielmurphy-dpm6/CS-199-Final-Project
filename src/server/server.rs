use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080...");

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break, // connection closed
            Ok(n) => {
                // broadcast received message to all connected clients
                tokio::io::stdout().write_all(&buf[..n]).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
