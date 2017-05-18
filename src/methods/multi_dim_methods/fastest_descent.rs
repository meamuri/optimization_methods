fn segment_div_help(x: f64, y: f64) -> f64 {
    x + y
}

pub fn fastest_descent(x: f64, y: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> (f64, f64) {
    let (mut x, mut y) = (x, y);

    loop {
        let grad_x = super::x_derivative(x, y, eps, f);
        let grad_y = super::y_derivative(x, y, eps, f);    
        if super::norm(grad_x, grad_y) < eps{
            break;
        }
        let alpha_new = segment_div_help(x, y);
        x -= alpha_new * grad_x;
        y -= alpha_new * grad_y;        
    }
    
    (x, y)
}