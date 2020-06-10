use crate::lib::{aabb::*, hittable::*, hittable_list::*, ray::*, vec3::*};

use rand::prelude::*;

use std::cmp::Ordering;
use std::sync::Arc;

enum BVHNode {
    Branch {
        left: Arc<dyn Hittable>,
        right: Arc<dyn Hittable>,
    },
    Leaf(Arc<dyn Hittable>),
}
pub struct BVH {
    tree: BVHNode,
    //using the word container since 'box' is a reserved keyword in Rust
    pub container: AABB,
}

impl BVH {
    pub fn new(
        mut objects: Vec<Arc<dyn Hittable>>,
        time_0: f64,
        time_1: f64,
        start: usize,
        end: usize,
    ) -> Self {
        //returning Ordering because the standard library's vector sort function expects an Ordering function
        #[allow(unused_assignments)]
        fn box_compare(
            a: Arc<dyn Hittable>,
            b: Arc<dyn Hittable>,
            time_0: f64,
            time_1: f64,
            axis_index: usize,
        ) -> Ordering {
            let mut x: bool;

            let mut a_container = AABB::default();
            x = a.bounding_box(time_0, time_1, &mut a_container);
            let mut b_container = AABB::default();
            x = b.bounding_box(time_0, time_1, &mut b_container);

            let ac: f64;
            let bc: f64;

            if axis_index == 1 {
                ac = a_container.min().x() + a_container.max().x();
                bc = b_container.min().x() + b_container.max().x();
            } else if axis_index == 2 {
                ac = a_container.min().y() + a_container.max().y();
                bc = b_container.min().y() + b_container.max().y();
            } else {
                ac = a_container.min().z() + a_container.max().z();
                bc = b_container.min().z() + b_container.max().z();
            }

            ac.partial_cmp(&bc).unwrap()
        }

        let mut rng = rand::thread_rng();

        let axis_index: usize = rng.gen_range(0, 3);
        let object_span = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = left.clone();
        } else if object_span == 2 {
            if box_compare(
                objects[start].clone(),
                objects[start + 1].clone(),
                time_0,
                time_1,
                axis_index,
            ) == Ordering::Greater
            {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_unstable_by(|a, b| {
                box_compare(a.clone(), b.clone(), time_0, time_1, axis_index)
            });

            let mid = start + object_span / 2;

            right = Arc::new(BVH::new(
                objects.drain(objects.len() / 2..).collect(),
                time_0,
                time_1,
                mid,
                end,
            ));
            left = Arc::new(BVH::new(objects, time_0, time_1, start, mid));
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        if !left.bounding_box(time_0, time_1, &mut box_left)
            || !right.bounding_box(time_0, time_1, &mut box_right)
        {
            panic!("No bounding box in BVHNode constructor \n");
        };

        let container = AABB::surrounding_box(&box_left, &box_right);

        BVH {
            tree: BVHNode::Branch { left, right },
            container,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        if !self.container.hit(ray, t_min, t_max) {
            false
        } else {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(ray, t_min, t_max, hit_rec),
                BVHNode::Branch { left, right } => {
                    let hit_left = left.hit(ray, t_min, t_max, hit_rec);
                    let hit_right = right.hit(ray, t_min, t_max, hit_rec);

                    hit_left || hit_right
                }
            }
        }
    }

    fn bounding_box(&self, t_0: f64, t_1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.container;
        true
    }
}
