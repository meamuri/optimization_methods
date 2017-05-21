use super::{x_derivative, y_derivative};


fn segment_divide_help(x: f64, y: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> f64 {
    let (mut a, mut b) = (-10_000.0, 10_000.0);    
    while b - a >= eps {
        let delta = (b - a) / 16.0;
        let u1 = (b + a - delta) / 2.0;
        let u2 = (b + a + delta) / 2.0;
        if help_func(x, y, u1, eps, f) < help_func(x, y, u2, eps, f) {
            b = u2;
        } else {
            a = u1;
        }
    }
    (a + b) / 2.0
}

fn help_func(x: f64, y: f64, alpha: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> f64 {
    f(x - alpha * x_derivative(x, y, eps, f), 
      y - alpha * y_derivative(x, y, eps, f))
}

pub fn fastest_descent(x: f64, y: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> (f64, f64) {
    let (mut x, mut y) = (x, y);

    loop {
        let grad_x = super::x_derivative(x, y, eps, f);
        let grad_y = super::y_derivative(x, y, eps, f);    
        if super::norm(grad_x, grad_y) < eps{
            break;
        }
        let alpha_new = segment_divide_help(x, y, eps, f);
        x -= alpha_new * grad_x;
        y -= alpha_new * grad_y;        
    }
    
    (x, y)
}