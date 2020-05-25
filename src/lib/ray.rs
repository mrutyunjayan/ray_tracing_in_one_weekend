use crate::lib::vec3::*;
#[derive(Copy, Clone, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: &Point3, direction: &Point3, time: f64) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
            time,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn set_origin(&mut self, origin: &Point3) {
        self.orig = *origin;
    }

    pub fn set_direction(&mut self, direction: &Point3) {
        self.dir = *direction;
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
