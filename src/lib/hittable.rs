use crate::lib::ray::Ray;
use crate::lib::vec3::*;

#[derive(Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

#[allow(dead_code)]
impl HitRecord {
    //create am invalid hit record
    pub fn new_invalid() -> Self {
        Self {
            p: Point3::new(-1.0, -1.0, -1.0),
            normal: Vec3::new(-1.0, -1.0, -1.0),
            t: -1.0,
            front_face: false,
        }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;

        //calculate the direction of the normal
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}
#[allow(unused_variables)]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool{
        false
    }
}
