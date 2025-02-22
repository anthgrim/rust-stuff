fn main() {
    // Owner
    let s1: String = String::from("RUST");

    let length = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, length);

    // Transfers the ownership of s1 to s2
    let s2 = s1;

    // This will throw an error because s1 is no longer valid
    // println!("The length of '{}' is {}", s1, length);

    println!("The length of '{}' is {}", s2, length);
}

/**
Function to calculate the length of a string
* Takes the reference of a string and return its length
*/
fn calculate_length(string: &String) -> usize {
    string.len()
}
