fn main() {
    //initialization of tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}",tuple);

    //initialization of tuple without data type
    let no_datatype_tuple  = ("Rust","fun", 100);
    println!("Tuple contents = {:?}",tuple);

    //accessing tuple elements at index 0
    println!("Value at index 0 = {}",datatype_tuple.0);

    //accessing tuple elements at index 1
    println!("Value at index 1 = {}",datatype_tuple.1);

    //accessing tuple elements at index 2
    println!("Value at index 2 = {}",datatype_tuple.2);

}
