/**
 * Entry point
 */
fn main() {
    tell_height(5);
    human_id("Kevin", 27, 5.7);
    functions::hello_world();
    let x = get_sum(1, 2);
    println!("Sum: {}", x);

    let _some = {
        let price = 5;
        let qty = 10;

        price * qty
    };

    println!("Some: {}", _some);

    println!("BMI: {}", get_bmi(5.7, 70.0));
}

/**
 * Function to print the height
 */
fn tell_height(height: i32) {
    println!("The height is: {}", height);
}

/**
 * Function to print the human id
 */
fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
}

// Expressions and Statements
fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

/**
 * Function to calculate the BMI
 */
fn get_bmi(height: f32, weight: f32) -> f32 {
    weight / height
}
