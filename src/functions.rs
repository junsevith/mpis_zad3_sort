pub fn identity(_n: f64, val: f64) -> f64 {
    return val;
}

pub fn div_n(n: f64, val: f64) -> f64 {
    let div = n;
    return val / div;
}

pub fn div_nsq(n: f64, val: f64) -> f64 {
    let div = n*n;
    return val / div;
}