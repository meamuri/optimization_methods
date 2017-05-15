pub fn custom_fn(x: f64) -> f64 {
    x * x - 4.0 * x + 10.0
}

pub fn ivan_fn(x: f64) -> f64 {
    9.0 * x.powf(6.0) - 51.0 * x.powf(5.0) - 29.0 * x.powf(3.0)
}

pub fn line_fn(x: f64) -> f64 {
    14.6125 * x + 1.0
}

pub fn simple_sqr_with_offset(x: f64) -> f64 {
    (x - 3.0) * (x - 3.0)
}