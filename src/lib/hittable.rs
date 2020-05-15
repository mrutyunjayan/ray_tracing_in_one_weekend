use crate::lib::{color::*, material::Material, ray::Ray, vec3::*};
#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Material,
    t: f64,
    front_face: bool,
}

#[allow(dead_code)]
impl HitRecord {
    //create am invalid hit record
    pub fn new_invalid() -> Self {
        Self {
            point: Point3::new(-1.0, -1.0, -1.0),
            normal: Vec3::new(-1.0, -1.0, -1.0),
            material: Material::default(),
            t: -1.0,
            front_face: false,
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_p(&mut self, point: Point3) {
        self.point = point;
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
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

    pub fn normal_to_color(&self) -> Color {
        Color::new(self.normal.x(), self.normal.y(), self.normal.z())
    }
}
#[allow(unused_variables)]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        false
    }
}
