fn main() {
    let A:i32 = 10;
    let B:i32 = 20;

    println!("Value of A: {} ",B);
    println!("Value of B: {} ",B);

    let mut res = A>B ;
    println!("A greater than B: {} ",res);

    res = A<B ;
    println!("A leser than B: {} ",res);

    res = A>=B ;
    println!("A greater than or equal to B: {} ",res);

    res = A<=B;
    println!("A leser than or equal to B: {}",res);

    res = A==B ;
    println!("A is equal to B: {}",res);

    res = A!=B ;
    println!("A is not equal to B: {} ",res);

}
