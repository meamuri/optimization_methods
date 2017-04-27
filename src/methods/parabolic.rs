// pub fn parabolic_method(_a: f64, _b: f64, u0: f64, eps: f64, f: &Fn(f64) -> f64) -> f64 {
//     let mut (a, b) = (_a, _b);
//     let i = 2;
//     let h = (b - a) / 16.0;

//     let mut t0 = u0;
//     let mut t1 = t0 + h;
//     let mut t2 = 0.0;

//     loop {
//         let mut I0 = f(t0);
//         let mut I1 = f(t1)

//         if I1 <= I0 {
//             t2 = t0 + h * (2.powi(i))            
//         } else {
//             t1 = t0 - h;
//             I1 = f(t1);
//             t2 = t0 - h * (2.powi(i))
//         }
//         I2 = f(t2);

//         if (t0 - t2).abs() < eps || (I0 - I2).abs() < eps {
//             return t0;
//         }

//         if t2 >= a && t2 <= b {
//             if is_convex(t0, t1, t2, f) {
//                 return W(t0, t1, t2);
//             } else {
//                 t0 = t2;
//                 t1 = t2 + h;
//                 i += 1;
//             }        
//         } else {
//             let w = if t2 < a {a} else {b};
//             if (t0 - w).abs() < eps {
//                 return t0;
//             }
            
//             // let mut In = 
//             //
//         }
//     }
// }


fn is_convex(u1: f64, u2: f64, u3: f64, f: &Fn(f64) -> f64) -> bool {
    let d1 = f(u1) - f(u2);
    let d2 = f(u3) - f(u2);
    return d1 >= 0.0 && d2 >= 0.0 && d1 + d2 > 0.0;
}

fn W(u1: f64, u2: f64, u3: f64, f: &Fn(f64) -> f64 ) -> f64 {
    let I1 = f(u1);
    let I2 = f(u2);
    let I3 = f(u3);
    return -0.5 * (
        (I2 - I1) * u3 * u3 + 
        (I1 - I3) * u2 * u2 + 
        (I3 - I2) * u1 * u1) / (
            (I1 - I2) * u3 + 
            (I3 - I1) * u2 + 
            (I2 - I3) * u1)
}