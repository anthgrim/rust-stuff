fn main() {
    // Loop keyword

    loop {
        println!("Hello World!");
        break;
    }

    // While loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Loop labels
    // For nested loops
    'outer: loop {
        println!("Entered the outer loop");

        loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }

    // While
    let mut number = 0;

    while number <= 5 {
        println!("The number is {}", number);
        number += 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
