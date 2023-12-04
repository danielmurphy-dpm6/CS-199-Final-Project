use std::net::{TcpListener, TcpStream};
use tokio::net::TcpListener as TokioTcpListener;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use once_cell::sync::Lazy;

mod message;
mod user;

use message::Message;
use user::User;

static USERS: Lazy<Mutex<HashMap<String, User>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on 127.0.0.1:8080...");

    while let Ok((stream, _)) = listener.accept() {
        tokio::spawn(handle_client(stream));
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut username = String::new();
    let mut password = String::new();

    if let Ok(_) = stream.read_to_string(&mut username).await {
        if let Ok(_) = stream.read_to_string(&mut password).await {
            // user registration
            let mut users = USERS.lock().await;
            if !users.contains_key(&username) {
                let new_user = User::new(&username, &password);
                users.insert(username.clone(), new_user.clone());
                println!("User '{}' registered successfully", new_user.get_name());
                // tell the client about successful registration
                let _ = stream.write_all(b"Registration successful").await;
            } else {
                // tell the client that the username is already taken
                let _ = stream.write_all(b"Username already taken").await;
            }
        } else {
            println!("Error reading password");
        }
    } else {
        println!("Error reading username");
    }
}
