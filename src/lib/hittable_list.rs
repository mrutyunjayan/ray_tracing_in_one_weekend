use crate::lib::{aabb::*, hittable::*, ray::Ray};
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

    pub fn bounding_box(&self, t_0: f64, t_1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut first_box = true;
        let mut temp_box = AABB::default();

        //iterate through all the boxues, expanding the "running box" to enclose each new one. In the end you have something
        //that surrounds all of them
        for object in &self.objects {
            //fill in temp_box with what is the bounding box of "object", and if this is impossible, return false
            if !object.bounding_box(t_0, t_1, &mut temp_box) {
                return false;
            }

            //if it is the first iteration, make the box of the whole list so far temp_box (which we just computed)
            *output_box = if first_box {
                temp_box

            //otherwise call surrdounding box and assign the result to the output box
            } else {
                AABB::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
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
