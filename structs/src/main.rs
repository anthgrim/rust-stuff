struct User {
    username: String,
    email: String,
}

fn main() {
    // tuple
    let rectangle: (i32, i32) = (30, 50);
    println!("{}", rectangle.0);

    let mut user_1 = build_user(String::from("email@email.com"), String::from("username"));

    user_1.email = String::from("email@email2.com");

    println!("{} is the username", user_1.username);
    println!("{} is the email", user_1.email);
}

fn build_user(email: String, username: String) -> User {
    User { email, username }
}
