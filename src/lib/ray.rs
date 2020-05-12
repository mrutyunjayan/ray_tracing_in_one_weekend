use super::vec3::*;
#[derive(Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}