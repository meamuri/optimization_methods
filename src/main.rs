extern crate optimization_methods; 

use optimization_methods::methods::single_dim_methods::parabolic;
use optimization_methods::methods::multi_dim_methods::{fastest_descent, splitting_of_step};
use optimization_methods::dialogs::{user_input, input2f64};
use std::f64::consts;

fn main() {
    let input = user_input("Введите:\n\
                            1. Метод парабол \n\
                            2. Многомерные методы.\n");
	match input.trim().parse::<i32>(){
        Ok(x) => {
            match x {
                1 => single_case(),
                2 => multi_case(),
                _ => println!("Введено некорректное число!"),
            }
        }        
        Err(..) => println!("Не введено целое число!!"),
    }
}

fn single_case() {
    let mut a   : 	f64 = 0.0;
	let mut b   : 	f64 = 0.0;
	let mut eps : 	f64 = 0.0;
    
    // let f = &|x: f64| -> f64 { x*x*x*x - x*x } ;
    let f = &|x: f64| -> f64 { (x + consts::PI).cos() } ;
	
    let mut input = user_input("Введите a (левая граница):");
	input2f64(&input, &mut a);	
 
	input = user_input("Введите b (правая граница):");
	input2f64(&input, &mut b);	

	input = user_input("Введите eps (погрешность):");
	input2f64(&input, &mut eps);	

    let res = parabolic::parabolic_method(a, b, eps, f );    
    println!("метод парабол \t\t {:?}", res);    
    println!("функция в этой точке: \t {:?}", f(res));    
}

fn multi_case() {
    let (mut x, mut y) = (0.0, 0.0);	
	let mut eps = 0.0;
    let mut alpha = 0.0;
    
    // let f = &|x: f64, y: f64| -> f64 { 3.0 * x * x + 4.0 * y * y + 1.0 } ;
	let f = &|x: f64, y: f64| -> f64 { 
        2.0 * x * x + 2.0 * y * y + 2.0 * x * y + 2.0 * x + 10.0 * y + 10.0
    } ;
    


    let mut input = user_input("Введите x:");
	input2f64(&input, &mut x);	
 
	input = user_input("Введите y:");
	input2f64(&input, &mut y);	

	input = user_input("Введите eps (погрешность):");
	input2f64(&input, &mut eps);	

    input = user_input("Введите alpha (начальный шаг):");
	input2f64(&input, &mut alpha);	

    let (res_x, res_y) = splitting_of_step::splitting_of_step(x, y, alpha, eps, f);    
    println!("Метод дробления шага:\n\
        x:\t {:?}\n\
        y:\t {:?}\n\n", res_x, res_y);
    
    let (res_x, res_y) = fastest_descent::fastest_descent(x, y, eps, f);    
    println!("Метод скорейшего спуска:\n\
        x:\t {:?}\n\
        y:\t {:?}\n\n", res_x, res_y);

    println!("Результат работы функции: {:?}", f(res_x, res_y));
}
