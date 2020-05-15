use crate::lib::{hittable::*, material::Material, ray::Ray, vec3::*};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    pub fn new_hittable(center: Point3, radius: f64, material: Material) -> Rc<dyn Hittable> {
        Rc::new(Self {
            center,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        //define the sphere and the ray interaction
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        //if the ray hits the sphere
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;

            //if 't' is within the bounds
            //first root
            if temp < t_max && temp > t_min {
                hit_rec.set_t(temp);
                hit_rec.set_p(ray.at(hit_rec.t()));
                let outward_normal = (hit_rec.point() - self.center) / self.radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.set_material(self.material);

                return true;
            }
            //second root
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_rec.set_t(temp);
                hit_rec.set_p(ray.at(hit_rec.t()));
                let outward_normal = (hit_rec.point() - self.center) / self.radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.set_material(self.material);

                return true;
            }
        }
        false
    }
}
