use std::io;

fn trapezium() {
    println!("You have selected Area of a Trapezium
            \ninput the height:");
   let mut height = String::new();
   io::stdin().read_line(&mut height).expect("Failed to read input");
   let height:f32 = height.trim().parse().expect("Failed to read input");
   
   println!("input the base 1:");
   let mut base1 = String::new();
   io::stdin().read_line(&mut base1).expect("Failed to read input");
   let base1:f32 = base1.trim().parse().expect("Failed to read input");

   println!("input the base 2:");
   let mut base2 = String::new();
   io::stdin().read_line(&mut base2).expect("Failed to read input");
   let base2:f32 = base2.trim().parse().expect("Failed to read input");

   let area_of_trapezium = height / 2.0 * (base1 + base2);

   println!("Area of Trapezium = {}", area_of_trapezium);
   
   }

fn rhombus(){
    println!("You have Selected Area of a Rhombus
            \ninput diagonal 1:");
   let mut diagonal1 = String::new();
   io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
   let diagonal1:f32 = diagonal1.trim().parse().expect("Failed to read input");
   
   println!("input diagonal 2");
   let mut diagonal2 = String::new();
   io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
   let diagonal2:f32 = diagonal2.trim().parse().expect("Failed to read input");

   let area_of_rhombus = 0.5 * diagonal1 * diagonal2;

   println!("Area of Rhombus = {}", area_of_rhombus);
   }

fn parallelogram() {    
    println!("You have Selected Area of a Parallelogram
            \ninput the base");
   let mut base = String::new();
   io::stdin().read_line(&mut base).expect("Failed to read input");
   let base:f32 = base.trim().parse().expect("Failed to read input");
   
   println!("input the altitude");
   let mut altitude = String::new();
   io::stdin().read_line(&mut altitude).expect("Failed to read input");
   let altitude:f32 = altitude.trim().parse().expect("Failed to read input");

   let area_of_parallelogram = base * altitude;

   println!("Area of Parallelogram = {}", area_of_parallelogram);
   }

fn cube() {    
    println!("You have Selected Area of a Cube
            \ninput the length of the side");
   let mut length = String::new();
   io::stdin().read_line(&mut length).expect("Failed to read input");
   let length:f32 = length.trim().parse().expect("Failed to read input");
  
   let area_of_cube1 = length.powf(2.0);
   let area_of_cube = 6.0 * area_of_cube1;

   println!("Area of Cube = {}", area_of_cube);
   }

fn cylinder(){    
    println!("You have Selected Volume of a Cylinder
            \ninput the radius");
   let mut radius = String::new();
   io::stdin().read_line(&mut radius).expect("Failed to read input");
   let radius:f32 = radius.trim().parse().expect("Failed to read input");
   
   println!("input the height");
   let mut height1 = String::new();
   io::stdin().read_line(&mut height1).expect("Failed to read input");
   let height1:f32 = height1.trim().parse().expect("Failed to read input");

   let pi = 22.0 / 7.0;
   let vol_of_cylinder_1 = radius.powf(2.0);
   let vol_of_cylinder = pi * vol_of_cylinder_1 * height1;

   println!("Volume of Cylinder = {}", vol_of_cylinder);
   }

   fn main() {
    loop{
   println!("What problem do you want to solve?
            \n1. Area of Trapezium
            \n2. Area of Rhombus
            \n3. Area of Parallelogram
            \n4. Area of Cube
            \n5. Volume of Cylinder
            \nPlease select a number");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let input1:i32 = input1.trim().parse().expect("Failed to read input");

   let arr = [trapezium,rhombus,parallelogram,cube,cylinder];

   if input1 >= 1 && input1 <= 5 {
    arr[(input1 - 1) as usize]();
   break;
    }

   else {
    println!("Invalid Input");
    continue;
   }
}}