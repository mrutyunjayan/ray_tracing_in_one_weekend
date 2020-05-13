use crate::lib::ray::*;
use crate::lib::vec3::*;

#[allow(dead_code)]
pub struct Camera {
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.25, 0.0),
            origin: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
