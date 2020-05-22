use crate::lib::{hittable::*, ray::Ray};
use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        //initializing temp_hit_rec with a value with the understanding that all implementations of the 'hit()' function will set it's own values for HitReacord instances before reading from them
        let mut temp_hit_rec = HitRecord::new_invalid();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            //default implementation of hit() is false
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_rec) {
                hit_anything = true;
                closest_so_far = temp_hit_rec.t();
                *hit_rec = temp_hit_rec;
            }
        }
        hit_anything
    }
}
