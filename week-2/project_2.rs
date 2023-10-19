fn main () {
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	//Sum
	let total = toshiba + mac + hp + dell + acer;
	println!("The Total amount of sales is {}", total);

	//Average
	let average = (toshiba + mac + hp + dell + acer)/5.0;
	println!("The average of total sales is {} ", average);



}