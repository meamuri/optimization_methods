//const k1 : f64 = 3_f64 - 5_f64.sqrt();

fn U1(a: f64, b: f64) -> f64 {
    let k1 = (3.0 - 5_f64.sqrt()) / 2.0;
    return k1 * (b - a) + a;
}

fn U2(a: f64, b: f64) -> f64 {
    let k2 = (5_f64.sqrt() - 1.0) / 2.0;
    return k2 * (b - a) + a;
}

pub fn golden_section(_a: f64, _b: f64, eps: f64, f: &Fn(f64) -> f64) -> f64 {
    let mut a = _a;
    let mut b = _b;

    let mut u1 = U1(_a, _b);
    let mut u2 = U2(_a, _b);

    while b - a >= eps {
        if f(u1) <= f(u2){
            b = u2;
            u2 = u1;
            u1 = U1(a, b);
        } else {
            a = u1;
            u1 = U1(a, b);
            u2 = U2(a, b);
        }
    }
    return (a + b) / 2.0;
}