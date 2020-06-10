use crate::lib::{aabb::*, hittable::*, material::Material, ray::Ray, vec3::*};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

pub struct MoveableSphere {
    center_time_start: Point3,
    center_time_end: Point3,
    radius: f64,
    material: Material,
    time_start: f64,
    time_end: f64,
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
    pub fn new_hittable(center: Point3, radius: f64, material: Material) -> Arc<dyn Hittable> {
        Arc::new(Self {
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

    fn bounding_box(&self, _t_0: f64, _t_1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &(self.center - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center + Vec3::new(self.radius, self.radius, self.radius)),
        );
        true
    }
}

impl MoveableSphere {
    pub fn new_hittable(
        center_time_start: Point3,
        center_time_end: Point3,
        radius: f64,
        material: Material,
        time_start: f64,
        time_end: f64,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self {
            center_time_start,
            center_time_end,
            radius,
            material,
            time_start,
            time_end,
        })
    }

    pub fn center(&self, time: f64) -> Point3 {
        let time_fraction = (time - self.time_start) / (self.time_end - self.time_start);
        self.center_time_start + time_fraction * (self.center_time_end - self.center_time_start)
    }
}

impl Hittable for MoveableSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        let center = self.center(ray.time());

        //define the sphere and the ray interaction
        let oc: Vec3 = ray.origin() - center;
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
                let outward_normal = (hit_rec.point() - center) / self.radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.set_material(self.material);

                return true;
            }
            //second root
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_rec.set_t(temp);
                hit_rec.set_p(ray.at(hit_rec.t()));
                let outward_normal = (hit_rec.point() - center) / self.radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.set_material(self.material);

                return true;
            }
        }
        false
    }

    fn bounding_box(&self, t_0: f64, t_1: f64, output_box: &mut AABB) -> bool {
        let (box_0, box_1): (AABB, AABB);

        box_0 = AABB::new(
            &(self.center(t_0) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(t_0) + Vec3::new(self.radius, self.radius, self.radius)),
        );

        box_1 = AABB::new(
            &(self.center(t_1) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(t_1) + Vec3::new(self.radius, self.radius, self.radius)),
        );

        *output_box = AABB::surrounding_box(&box_0, &box_1);

        true
    }
}
