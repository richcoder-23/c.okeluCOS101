//rust program to calculate speed 
fn main() {

   let distance_1:f64 = 80.0;
   let distance_2:f64 = 120.0;
   let mut _result:f64;

   let time_1:f64 = 2.0;
   let time_2:f64 = 4.0;
   let mut _result:f64;

   _result = (distance_1 * 1.61) / time_1;
   println!("Formula1: {}",_result);

   _result = (distance_2 * 1.61) / time_2;
   println!("NeedForSpeed: {}",_result);
}