use std::f64::consts::PI;

#[allow(dead_code)]
pub const INFINITY: f32 = f32::MAX; // 1.7976931348623157E+308f64

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

