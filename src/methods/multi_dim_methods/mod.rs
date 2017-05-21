pub fn x_derivative(x: f64, y: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> f64{
    let res_x_add = f(x + eps, y);
    let res_x_sup = f(x - eps, y); 
    (res_x_add - res_x_sup) / (2.0 * eps)
}

pub fn y_derivative(x: f64, y: f64, eps: f64, f: &Fn(f64,f64) -> f64) -> f64{
    let res_y_add = f(x, y + eps);
    let res_y_sup = f(x, y - eps); 
    (res_y_add - res_y_sup) / (2.0 * eps)
}

pub fn norm(x: f64, y: f64) -> f64 {
    (x*x + y*y).sqrt()
}

pub mod splitting_of_step;
pub mod fastest_descent;