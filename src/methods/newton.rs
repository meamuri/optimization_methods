use std::f64;

fn first_derivative(f: &Fn(f64)-> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - f(x)) / h;
}

fn second_derivative(f: &Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - 2.0*f(x) + f(x - h)) / h.powf(2.0);
}