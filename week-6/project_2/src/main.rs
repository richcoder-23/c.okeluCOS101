// Rust Program To Determine Incentive Of The Nigerian Researchers Publication Incentive System 

use std::io;

fn main() {
    for x in 1..501{
    println!("Researcher Incentive Calculator System");
    let mut input1 = String::new();
    


    println!("\nEnter the Name Of The Researcher: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not A Valid Input");

    println!("\nEnter The Number Of Papers Published:");
    io::stdin().read_line(&mut input1).expect("Not A Valid Number");
    let number_of_papers_published:i64 = input1.trim().parse().expect("Not A valid Number");


    if number_of_papers_published == 3 && number_of_papers_published >= 3 && number_of_papers_published == 5 && number_of_papers_published <= 5 {
     let incentive:f32 = 500_000.00;
     println!("You,{} have An Incentive of: N{}",name,incentive);

    } if number_of_papers_published >= 5 && number_of_papers_published <= 10{
        let incentive:f32 = 800_000.00;
        println!("You,{} have An Incentive of: N{}",name,incentive);
    
    } if number_of_papers_published == 10 && number_of_papers_published >= 10{
        let incentive:f32 = 1_000_000.00;
        println!("You,{} have An Incentive of: N{}",name,incentive);

    } if number_of_papers_published <= 3{
        let incentive:f32 = 100_000.00;
        println!("You,{} have An Incentive of: N{}",name,incentive);

   println!("count {} ", x);
   println!("");//just an empty line
  }   
 }
}

