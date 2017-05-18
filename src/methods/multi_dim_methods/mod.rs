pub fn x_derivative(x: f64, y: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> f64{
    f(x + eps, y) - f(x - eps, y) / (2.0 * eps)
}

pub fn y_derivative(x: f64, y: f64, eps: f64, f: &Fn(f64,f64) -> f64) -> f64{
    f(x, y + eps) - f(x, y - eps) / (2.0 * eps)
}

pub fn norm(x: f64, y: f64) -> f64 {
    (x*x + y*y).sqrt()
}

pub mod splitting_of_step;
pub mod fastest_descent;