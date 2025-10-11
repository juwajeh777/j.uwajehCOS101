fn main() {
	// item prices
	let toshiba:f64 = 450000.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 750000.0;
	let dell:f64 = 2850000.0;
	let acer:f64 = 250000.0;

	// item quantity
	let toshiba_qty:f64 = 2.0;
	let mac_qty:f64 = 1.0;
	let hp_qty:f64 = 3.0;
	let dell_qty:f64 = 3.0;
	let acer_qty:f64 = 1.0;

	// total quantity for each item
	let toshiba_total_qty:f64 = toshiba * toshiba_qty;
	let mac_total_qty:f64 = mac * mac_qty;
	let hp_total_qty:f64 = hp * hp_qty;
	let dell_total_qty:f64 = dell * dell_qty;
	let acer_total_qty:f64 = acer * acer_qty;

	// formula for sum, total quantity and average
	let sum = toshiba_total_qty + mac_total_qty + hp_total_qty + dell_total_qty + acer_total_qty;
	let total_qty = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
	let average = sum / total_qty;

	println!("Total sum is equal to {} and Average is equal to {}", sum,average);

}