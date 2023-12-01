extern crate chrono;

use crate::user::User;

#[derive(Debug)]
struct MessageMetaData<'a> {
    user: &'a User<'a>,
    time: Option<chrono::DateTime<chrono::Utc>>, // Use Option directly
}

#[derive(Debug)]
pub struct Message<'a> {
    text: String,
    send_meta: MessageMetaData<'a>,
    receive_meta: MessageMetaData<'a>,
}