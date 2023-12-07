mod user;

fn main() {
    let user1 = user::User::new("saad", "saad04");
    print!("name: {}", &user1.get_name())
}