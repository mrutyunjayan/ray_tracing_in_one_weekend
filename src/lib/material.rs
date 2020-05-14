use crate::lib::{color::*, hittable::*, ray::*};

#[allow(unused_variables)]
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        false
    }
}
