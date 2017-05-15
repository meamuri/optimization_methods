pub fn segment_divide(
	_a: 	f64, 
	_b: 	f64, 
	eps: 	f64, 
	f: 		&Fn(f64) -> f64
	) 		-> f64 {

	let mut a = _a;
	let mut b = _b;	

	while b - a >= eps {
		let delta 	= (b - a) / 4.0;
		let u1 		= (b + a - delta) / 2.0;
		let u2 		= (b + a + delta) / 2.0;
		if f(u1) <= f(u2) {
			b = u2;
		} else {
			a = u1;
		}		
	}
	(a + b) / 2.0
}