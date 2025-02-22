fn main() {
    // Vectors
    let mut vector: Vec<i32> = Vec::new();
    let mut initialized_vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Append
    vector.push(5);
    initialized_vector.push(10);

    println!("{:?}", initialized_vector);
}
