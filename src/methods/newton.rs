use std::f64;
use std::num;

fn first_derivative(f: &Fn(f64)-> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - f(x)) / h;
}

fn second_derivative(f: &Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - 2.0*f(x) + f(x - h)) / h.powf(2.0);
}

pub fn newton_method(a: f64, b: f64, _u0: f64, eps: f64, f: &Fn(f64) -> f64) -> f64 {
    let mut u0 = _u0;
    while first_derivative(f, u0, eps).abs() >= eps && u0 >= a && u0 <= b {
            u0 -= first_derivative(f, u0, eps) / second_derivative(f, u0, eps);
        }
    if u0 >= a && u0 <= b {
        return u0;
    }
    return if u0 <= a { a } else { b };
}