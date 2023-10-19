fn main() {
    let principal = 210_000.0;
    let rate = 5.0;
    let years:i32 = 3;

    // Depreciation
    let amount = principal * (1.0 - (rate as f64 / 100.0)).powi(years);

    println!("The Value of the TV is {}", amount);
}