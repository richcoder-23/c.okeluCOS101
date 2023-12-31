// Rust Program To Derivive Roots For 


use std::io;

fn main() {
    println!("Enter value of a");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Not a valid number");
    let a:f32 = input_1.trim().parse().expect("Not a valid input");


    println!("Enter a value of b");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Not a Valid number");
    let b:f32 = input_2.trim().parse().expect("Not a valid input");


    println!("Enter a value of c");
    let mut input_3 = String::new();
    io::stdin().read_line(&mut input_3).expect("Not a valid number");
    let c:f32 = input_3.trim().parse().expect("Not a vald unit");

    // Calculate the discriminant 
    let d:f32 = b.powf(2.0) - 4.0 * a * c;


    if d:f32 > 0.0 
    {
        // Two distinct real roots 
        let root1 = (-b + d.sqrt(2.0)) / (2.0 * a);
        let root2 = (-b - d.sqrt(2.0)) / (2.0 * a);
        {
        println!("When The dicriminant is greater than Zero: Root1 = {}, Root2 = {}",root1,root2);
        }
    }
        else if d:f32 == 0.0
        {
        // One Real root   
        let root = -b / (2.0 * a);
        {
        println!("Since The dicriminant is Equal To Zero: Root = {}",root);
        }
    }
        else {
        // Imaginary / No real roots 
        println!("No real roots (discriminant is negative).");
             }

}
