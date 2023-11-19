use std::io;


fn main() {
    
    //list out variables
   
   println!("what number are you?");
   let mut input_1 = String::new();
   io::stdin().read_line(&mut input_1).expect("error");
   let batch_no:i32 = input_1.trim().parse().expect("not a valid number");

   println!("what's your name?");
   let mut input_2 = String::new();
   io::stdin().read_line(&mut input_2).expect("error ");
   let name:String = input_2.trim().parse().expect("invalid");

   println!("what's your email?");
   let mut input_3 = String::new();
   io::stdin().read_line(&mut input_3).expect("error ");
   let email:String= input_3.trim().parse().expect("invalid email");

   println!("what department are you in?");
   let mut input_4 = String::new();
   io::stdin().read_line(&mut input_4).expect("error ");
   let department:String= input_4.trim().parse().expect("invalid email");

   if !email.contains('@') {
        println!("Invalid email format");
   }

  if batch_no >150 {
    println!("Voting has closed");
  }
  println!("Are you a class rep?");
  let mut input_5 = String::new();
  io::stdin().read_line(&mut input_5).expect("error");
  let class_rep:String = input_5.trim().to_lowercase();

  if class_rep == "no "
    {
        println!("Are you in 100 level?");
        let mut input_6 = String::new();
        io::stdin().read_line(&mut input_6).expect("error");
        let level:String = input_6.trim().to_lowercase();
    }
    else {
        println!("You are not eligible!");
    }

}


