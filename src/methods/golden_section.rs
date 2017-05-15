fn fn_u1(a: f64, b: f64) -> f64 {
    let k1 = (3.0 - 5_f64.sqrt()) / 2.0;
    k1 * (b - a) + a
}

fn fn_u2(a: f64, b: f64) -> f64 {
    let k2 = (5_f64.sqrt() - 1.0) / 2.0;
    k2 * (b - a) + a
}

pub fn golden_section(_a: f64, _b: f64, eps: f64, f: &Fn(f64) -> f64) -> f64 {
    let mut a = _a;
    let mut b = _b;

    let mut u1 = fn_u1(_a, _b);
    let mut u2 = fn_u2(_a, _b);

    while b - a >= eps {
        if f(u1) <= f(u2){
            b = u2;
            u2 = u1;
            u1 = fn_u1(a, b);
        } else {
            a = u1;
            u1 = fn_u1(a, b);
            u2 = fn_u2(a, b);
        }
    }
    (a + b) / 2.0
}