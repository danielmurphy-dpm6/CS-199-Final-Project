extern crate argon2;
extern crate rand;
extern crate once_cell;

use std::sync::Mutex;

use once_cell::sync::Lazy;

mod message;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use rand::rngs::OsRng;
use std::collections::LinkedList;
use message::Message;

static ARGON2: Lazy<Mutex<Argon2>> = Lazy::new(|| Mutex::new(Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, argon2::Params::DEFAULT))); 
static NEXT_ID: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(1));

#[derive(Debug)]
pub struct User<'a> {
    user_name: String,
    id: u64,
    password_hash: String, 
    contacts: LinkedList<User<'a>>,
    messages: LinkedList<Message<'a>>,
}

impl User<'_> {    
    pub fn new(user_name: &str, password: &str) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = ARGON2.lock().unwrap().hash_password(password.as_bytes(), &salt).unwrap();

        Self {
            user_name: user_name.to_string(),
            id: User::get_next_id(),
            password_hash: password_hash.to_string(),
            contacts: LinkedList::new(),
            messages: LinkedList::new(),
        }
    }

    // Associated function to get the next ID
    fn get_next_id() -> u64 {
        let mut id = NEXT_ID.lock().unwrap();
        let current_id = *id;
        *id += 1;
        current_id
    }

    pub fn verify_password(&self, input_password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&input_password);
        match parsed_hash {
            Ok(val) => {
                ARGON2.lock().unwrap()
                .verify_password(self.password_hash.as_bytes(), &val)
                .is_ok()
            } 
            Err(_) => false
        }
    }

    pub fn get_name(&self) -> String {self.user_name.clone()}

    pub fn set_name(&self, new_name: String) {
        self.user_name = new_name;
    }
}