use crate::lib::{ray::*, vec3::*};

use partial_min_max::*;

#[derive(Copy, Clone, Default)]
pub struct AABB {
    _min: Point3,
    _max: Point3,
}

#[allow(dead_code)]
impl AABB {
    pub fn new(min: &Point3, max: &Point3) -> Self {
        Self {
            _min: *min,
            _max: *max,
        }
    }

    pub fn min(&self) -> Point3 {
        self._min
    }

    pub fn max(&self) -> Point3 {
        self._max
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let (mut t_0, mut t_1): (f64, f64);
        let mut inv_d: f64;

        //go through each axis
        for axis in 0..3 {
            match axis {
                0 => {
                    //x = 0
                    inv_d = 1.0 / ray.direction().x();

                    //𝑡𝑥0 = min((𝑥0 − 𝐴𝑥) / 𝑏𝑥, (𝑥1 − 𝐴𝑥) / 𝑏𝑥)
                    t_0 = min(
                        (self._min.x() - ray.origin().x()) * inv_d,
                        (self._max.x() - ray.origin().x()) * inv_d,
                    );

                    //𝑡𝑥0 = max((𝑥0 − 𝐴𝑥) / 𝑏𝑥, (𝑥1 − 𝐴𝑥) / 𝑏𝑥)
                    t_1 = max(
                        (self._min.x() - ray.origin().x()) * inv_d,
                        (self._max.x() - ray.origin().x()) * inv_d,
                    );
                }

                1 => {
                    //y = 1
                    inv_d = 1.0 / ray.direction().y();

                    //𝑡y0 = min((y0 − 𝐴y) / 𝑏y, (y1 − 𝐴y) / 𝑏y)
                    t_0 = min(
                        (self._min.y() - ray.origin().y()) * inv_d,
                        (self._max.y() - ray.origin().y()) * inv_d,
                    );

                    //𝑡y0 = max((y0 − 𝐴y) / 𝑏y, (y1 − 𝐴y) / 𝑏y)
                    t_1 = max(
                        (self._min.y() - ray.origin().y()) * inv_d,
                        (self._max.y() - ray.origin().y()) * inv_d,
                    );
                }
                2 => {
                    // z = 2
                    inv_d = 1.0 / ray.direction().z();

                    //𝑡z0 = min((z0 − 𝐴z) / 𝑏z, (z1 − 𝐴z) / 𝑏z)
                    t_0 = min(
                        (self._min.z() - ray.origin().z()) * inv_d,
                        (self._max.z() - ray.origin().z()) * inv_d,
                    );

                    //𝑡z0 = max((z0 − 𝐴z) / 𝑏z, (z1 − 𝐴z) / 𝑏z)
                    t_1 = max(
                        (self._min.z() - ray.origin().z()) * inv_d,
                        (self._max.z() - ray.origin().z()) * inv_d,
                    );
                }
                _ => continue,
            }

            //checking if the ray direction is reversed
            let (t_0, t_1) = if inv_d < 0.0 { (t_1, t_0) } else { (t_0, t_1) };

            let t_min = if t_0 > t_min { t_0 } else { t_min };
            let t_max = if t_1 < t_max { t_1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box_0: &AABB, box_1: &AABB) -> AABB {
        let (small, big): (Point3, Point3);

        small = Point3::new(
            min(box_0._min.x(), box_1._min.x()),
            min(box_0._min.y(), box_1._min.y()),
            min(box_0._min.z(), box_1._min.z()),
        );

        big = Point3::new(
            max(box_0._max.x(), box_1._max.x()),
            max(box_0._max.y(), box_1._max.y()),
            max(box_0._max.z(), box_1._max.z()),
        );

        AABB::new(&small, &big)
    }
}
