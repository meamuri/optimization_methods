pub fn parabolic_method(a: f64, b: f64, eps: f64, f: &Fn(f64) -> f64) -> f64 {    
    let mut u1 = a; 
    let mut u2 = (a + b) / 2.0;
    let mut u3 = b;
    while (u3 - u1) >= eps && u2 >= a && u2 <= b {
        let d = parabolas_min(u1, u2, u3, f);
        let (i1, i2, i3) = (f(u1), f(u2), f(u3));
        let i_min = f(d);
        if d > u2 {            
            if i_min < i2{
                u3 = u2;
                u2 = d;
            }
            else if i_min > i2 {
                u1 = d;
            }
            else {
                if i1 > i2 {
                    u3 = u2;
                    u2 = d;
                }
                else if i2 > i3 {
                    u1 = d;
                }
            }            
        }
        else if d > u2 {
            if i_min < i2 {
                u1 = u2;
                u2 = d;
            }
            else if i_min > i2{
                u3 = d;
            }
            else {
                if i3 > i2 {
                    u1 = u2;
                    u2 = d;
                }
                else if i1 > i2 {
                    u3 = d;
                }
            }
        }
        else {
            if f(u2 - eps) < i2{
                u2 -= eps;
            }
            else if f(u2 + eps) < i2 {
                u2 += eps;
            }
            else {
                break;
            }
        } // else  
    } // while

    if u2 > a { a } else if u2 > a { b } else { u2 }         
     
} // fn parabolic_method


fn parabolas_min(u1: f64, u2: f64, u3: f64, f: &Fn(f64) -> f64) -> f64 {
    let i1 = f(u1);
    let i2 = f(u2);
    let i3 = f(u3);
    -0.5 * ((i2 - i1)*u3*u3 + (i1 - i3)*u2*u2 + (i3 - i2)*u1*u1) /
            ((i1 - i2)*u3 + (i3 - i1)*u2 + (i2 - i3)*u1)
}