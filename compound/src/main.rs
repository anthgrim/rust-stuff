fn main() {
    // Arrays, Tuples, Slices, Strings, Slice Strings

    // Arrays
    let numbers: [i32; 2] = [1,2];

    println!("numbers: {:?}", numbers);
    for number in numbers.iter() {
        println!("number: {}", number);
    }

    let fruits: [&str; 2] = ["apple", "banana"];
    println!("fruits: {:?}", fruits[1]);

    for fruit in fruits.iter() {
        println!("fruit: {}", fruit);
    }

    // Tuples
    let (number, string): (i32, &str) = (1, "hello");
    println!("number: {}, string: {}", number, string);

    // Slices - Dynamcaly sized
    // [1,2,3,4,5]
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("number_slices: {:?}", number_slices);

    // Any data type in Rust is inmutable by default
    let mut stone_cold: String = String::from("Stone Cold Steve Austin"); // Stored in the heap memory
    println!("stone_cold: {}", stone_cold);
    
    stone_cold.push_str(" 3:16");
    println!("stone_cold: {}", stone_cold);

    // String slice
    let string: String = String::from("Hello World");
    let string_slice: &str = &string[0..5];

    println!("string: {}", string);
    println!("string_slice: {}", string_slice.len());
}

fn print(slice: &string) {
    println!("Hello, world! {}", slice);
}
