use std::collections::HashMap;
use tokio::time::Duration;
use tokio_console::input;

#[tokio::main]
async fn main() {
    let mut users = HashMap::new();

    // Adding some hardcoded users to test implmenentation
    users.insert("user1", "password1");
    users.insert("user2", "password2");

    loop {
        println!("Login to your account:");

        let username = input("Username: ").await.unwrap();
        let password = input("Password: ").await.unwrap();

        if let Some(expected_password) = users.get(&username) {
            if password == *expected_password {
                println!("Login successful! Welcome, {}.", username);
                // Would need to add some stuff here for what happens after successful login
                break;
            } else {
                println!("Incorrect password. Try again.");
            }
        } else {
            println!("User not found. Try again.");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}