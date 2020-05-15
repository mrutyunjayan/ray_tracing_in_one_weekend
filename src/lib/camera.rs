use crate::lib::{ray::*, vec3::*};
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.25, 0.0),
            lower_left_corner: Point3::new(-2.0, -1.125, -1.0),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin),
        )
    }
}
