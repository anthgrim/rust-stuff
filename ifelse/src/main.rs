fn main() {
    // If else
    let age: u16 = 18;

    let is_qualified = is_qualified(&age);

    if is_qualified {
        println!("You are qualified to vote");
    } else {
        println!("You are not qualified to vote");
    }
}

fn is_qualified(age: &u16) -> bool {
    if *age >= 18 {
        return true;
    } else {
        return false;
    }
}
