pub fn splitting_of_step(x: f64, y: f64, alpha: f64, eps: f64, f: &Fn(f64, f64) -> f64) -> (f64, f64) {
    let (mut x, mut y, mut alpha) = (x, y, alpha);
    let mut i0 = f(x, y);
    let mut grad_x = super::x_derivative(x, y, eps, f);
    let mut grad_y = super::y_derivative(x, y, eps, f);
    while super::norm(grad_x, grad_y) >= eps {
        let x1 = x - alpha * grad_x;
        let y1 = y - alpha * grad_y;
        let i1 = f(x1, y1);
        if i1 <= i0 {
            alpha /= 2.0;
        } else {
            x = x1;
            y = y1;
            i0 = i1;
        }
        grad_x = super::x_derivative(x, y, eps, f);
        grad_y = super::y_derivative(x, y, eps, f);
    }
    (x, y)
}