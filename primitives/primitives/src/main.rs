// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned (only +) integers
// Signed integers: i8, i16, i32, i64, i128, isize

fn main() {
    let x: i32 = -42;
    let y: u64 = 34;

    // Difference between i32 (32 bits) and u64 (64 bits)
    // Range from i32: -2^31 to 2^31 - 1
    // Range from u64: 0 to 2^64 - 1
    let max_i32: u64 = std::u64::MIN;

    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
    println!("Max i32 integer: {}", max_i32);

    // Floats

    // f32, f64
    let f1: f32 = 3.14;
    let f2: f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;

    println!("Float 1: {}", f1);
    println!("Float 2: {}", f2);

    // boolean
    let is_true: bool = true;
    let is_false: bool = false;

    println!("Boolean 1: {}", is_true);
    println!("Boolean 2: {}", is_false);

    // char
    // Single unicode character
    let c: char = 'a';
    let heart_eyed_cat: char = '😻';

    println!("Char 1: {}", c);
    println!("Char 2: {}", heart_eyed_cat);
}