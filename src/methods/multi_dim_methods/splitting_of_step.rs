use super::{x_derivative, y_derivative, norm};

pub fn splitting_of_step(x: f64, y: f64, alpha: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> (f64, f64) {
    let (mut x, mut y, mut alpha) = (x, y, alpha);
    let mut i0 = f(x, y);
    
    loop {
        let grad_x = x_derivative(x, y, eps, f);
        let grad_y = y_derivative(x, y, eps, f);

        if norm(grad_x, grad_y) < eps {
            break;
        }

        let x1 = x - alpha * grad_x;
        let y1 = y - alpha * grad_y;
        let i1 = f(x1, y1);
        if i1 >= i0 {
            alpha /= 2.0;
        } else {
            x = x1;
            y = y1;
            i0 = i1;
        }
    }
    (x, y)
}