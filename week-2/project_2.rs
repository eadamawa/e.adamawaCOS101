fn main() {

//variables for the prices
	let p_t:f64 = 450_000.00;
	let p_m:f64 = 1_500_000.00;
	let p_h:f64 = 750_000.00;
	let p_d:f64 = 2_850_000.00;
	let p_a:f64 = 250_000.00;

//variables for the quantity
    let q_t:f64 = 2.0;
	let q_m:f64 = 1.0;
	let q_h:f64 = 3.0;
	let q_d:f64 = 3.0;
	let q_a:f64 = 1.0;

//total price
	let total = (p_t*q_t) + (p_m*q_m) + (p_h*q_h) + (p_d*q_d) + (p_a*q_a);

//total quantity
	let total_qty = q_t + q_m + q_h + q_d + q_a;

    println!("Total is {}", total);

//average
    let average = total/total_qty;
    println!("Average is {}", average);
}
