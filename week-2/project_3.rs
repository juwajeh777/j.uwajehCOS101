fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// formula for depreciation
	let a = p * (1.0 - (r / 100.0)).powf(n);

	println!("Depreciated value of TV after 3 years is {}", a);
	
}