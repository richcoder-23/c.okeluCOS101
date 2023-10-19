fn main () {
	let p:f64 = 520_000_000.0;
	let n:f64 = 5.0;
	let r:f64 = 10.0;

	//Compound Interest
	let a = p *(1.0 + r/100.0)* n;
	let cl = a - p;
	println!("The Compound interest is {}", cl);

}