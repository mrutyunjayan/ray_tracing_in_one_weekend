use crate::lib::{ray::*, rt_math::*, vec3::*};

use rand::prelude::*;

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_0: f64, //shutter open time
    time_1: f64, //shutter close time
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        v_up: &Vec3,
        v_fov: f64, /*vertical field of view, in degrees*/
        aspect_ratio: f64,
        aperture: f64,
        focus: f64,
        time_0: f64,
        time_1: f64,
    ) -> Self {
        let theta = degrees_to_radians(v_fov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (*look_from - *look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = 2.0 * half_width * focus * u;
        let vertical = 2.0 * half_height * focus * v;
        let lower_left_corner =
            origin - half_width * focus * u - half_height * focus * v - focus * w;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            time_0,
            time_1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand_in_disk = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rand_in_disk.x() + self.v * rand_in_disk.y();

        let mut rng = rand::thread_rng();
        let time = rng.gen::<f64>() * (self.time_1 - self.time_0);

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset),
            time,
        )
    }
}
