//Rust program to calculate the roots of a 
// quadratic equation
 
use std::io;

fn main()
{
    println!("Enter the value of a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let c:f32 = input2.trim().parse().expect("Not a valid number");

    //calculate discriminant
    
}