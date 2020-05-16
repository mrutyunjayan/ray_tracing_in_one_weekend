use crate::lib::{ray::*, rt_math::*, vec3::*};
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.25, 0.0),
            lower_left_corner: Point3::new(-2.0, -1.125, -1.0),
        }
    }

    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        v_up: &Vec3,
        vfov: f64, /*vertical field of view, in degrees*/
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (*look_from - *look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        let lower_left_corner = origin - half_width * u - half_height * v - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin),
        )
    }
}
