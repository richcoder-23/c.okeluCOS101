use std::io;

fn add(a: i32, b:i32) {
    let sum = a + b;

    println!("Sum of A and B = {}",sum);
}

fn main() {

    let mut input_1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let a:i32 - input_1.trim().parse().expect("Invalid input");

    let mut input_2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let b:i32 - input_2.trim().parse().expect("Invalid input");

    //call add function with arguments
    add(a, b);

}
