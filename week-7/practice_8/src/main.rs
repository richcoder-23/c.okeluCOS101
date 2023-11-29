fn main() {
    let city_arr:[&str;5] = ["Abuja","Portharcout","Maiduguri","Kano","lagos"];
    println!("array is {:?}",arr1);
    println!("\narray size is :{}",city_arr.len());

    for index in 0..5 {
        println!("City index {} is located is : {}",index,city_arr[index]);
    }
}
