extern crate OptimizationMethods; 

use OptimizationMethods::methods::segment_div::segment_divide;
use OptimizationMethods::methods::golden_section::golden_section;
use OptimizationMethods::methods::newton::newton_method;

use OptimizationMethods::dialogs::{user_input, input2f64};
use OptimizationMethods::functions::{simple_sqr_with_offset, ivan_fn};

fn main() {
    let mut a   : 	f64 = 0.0;
	let mut b   : 	f64 = 0.0;
	let mut eps : 	f64 = 0.0;

    //let f = &simple_sqr_with_offset;
    let f = &|x: f64| -> f64 { x*x*x*x - x*x } ;
	
    let mut input = String::new(); 
    
    input = user_input("Введите a (левая граница):");
	input2f64(&input, &mut a);	
 
	input = user_input("Введите b (правая граница):");
	input2f64(&input, &mut b);	

	input = user_input("Введите eps (погрешность):");
	input2f64(&input, &mut eps);	

    let mut res = segment_divide(a, b, eps, f );    
    println!("метод деления отрезка \t {:?}", res);

    res = golden_section(a, b, eps, f );
    println!("метод золотого сечения \t {:?}", res);
    
    let mut u0 : 	f64 = 0.0;

    input = user_input("Введите начальную точку:");
	input2f64(&input, &mut u0);

    res = newton_method(a, b, u0, eps, f );
	println!("метод Ньютона \t {:?}", res);
}
