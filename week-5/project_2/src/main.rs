//rust program to allocate employee incentive

use std::io;

fn main() {
    
    loop{

    println!("\nDo you have experience? (Yes or no): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    let experience = experience.trim().to_lowercase();

    println!("\nHow old are you ? : ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:i32 = age.trim().parse().expect("Not a valid number");

        
        if experience == "yes"{
        if age >= 40{
            println!("Your incentive is 1,560,000");
    }

    else if age >= 30 && age < 40 
    {
       println!("Your incentive is 1,480,000");
    }

    else if age < 28 
    {
       println!("Your incentive is 1,300,000");
    }

    }
    else if experience == "no" {
        println!("Your incentive is 1,000,000");
    }
    
    else 
    {
    println!("error, Try again :) " );
    }
    break;
}
        
}
    
    