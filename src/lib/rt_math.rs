#[allow(dead_code)]
pub const PI: f64 = 3.1415926535897932385;
#[allow(dead_code)]
pub const INFINITY: f64 = f64::MAX; // 1.7976931348623157E+308f64

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[allow(dead_code)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}
