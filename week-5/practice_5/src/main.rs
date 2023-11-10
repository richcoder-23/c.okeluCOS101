//Rust program to read the height of a person
// and then print if the person is tall, dwarf,
// or average height person

use std::io;

fn main()
{
   let mut input1 = String::new();

    println!("\nEnter your height (in centimeters):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f32 = input1.trim().parse().expect("Not a valid number");
    
    if height >= 150.0 && height <= 170.0
    {
        println!("You are an average height person");
    }
    else if height > 170.0 && height <=195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf");
    }
    else
    {
        println!("Your height is abnormal");
    } 
}
