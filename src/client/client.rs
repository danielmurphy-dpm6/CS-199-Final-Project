use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to the server...");

    // user registration (replace with your desired username and password)
    let username = "user123";
    let password = "password123";

    if let Err(e) = register_user(&stream, username, password).await {
        println!("Error during registration: {}", e);
        return;
    }

    println!("Registration successful!");

    // client logic continues here...
    // add code to send and receive messages, etc.
}

async fn register_user(stream: &TcpStream, username: &str, password: &str) -> io::Result<()> {
    // Send the username and password to the server for registration
    stream.write_all(username.as_bytes()).await?;
    stream.write_all(password.as_bytes()).await?;

    // the server's response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    // the server's response
    println!("{}", response);

    Ok(())
}
