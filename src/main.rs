extern crate OptimizationMethods; 

use OptimizationMethods::methods::segment_div::segment_divide;
use OptimizationMethods::dialogs::{user_input, input2f64};

fn main() {
    let mut a : 	f64 = 0.0;
	let mut b : 	f64 = 0.0;
	let mut eps : 	f64 = 0.0;

	let mut input = user_input("Введите a:");
	input2f64(&input, &mut a);
	println!("{}", a);

	input = user_input("Введите b:");
	input2f64(&input, &mut b);
	println!("{}", b);

	input = user_input("Введите eps:");
	input2f64(&input, &mut eps);
	println!("{}", eps);

	let res = segment_divide(a, b, eps, &|x: f64| -> f64 { x*x } );
	println!("{:?}", res);
}
