use std::f64::consts::*;
use std::ops;

use rand::prelude::*;

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub trait Vec3Traits {
    type Kind;

    fn length_squared(&self) -> f64 {
        self.first_element() * self.first_element()
            + self.second_element() * self.second_element()
            + self.third_element() * self.third_element()
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn dot(&self, rhs: &Self) -> f64 {
        self.first_element() * rhs.first_element()
            + self.second_element() * rhs.second_element()
            + self.third_element() * rhs.third_element()
    }
    fn unit_vector(&self) -> Self::Kind;
    fn cross(&self, rhs: &Self::Kind) -> Self::Kind;
    fn first_element(&self) -> f64;
    fn second_element(&self) -> f64;
    fn third_element(&self) -> f64;
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p: Vec3;
        let mut rng = rand::thread_rng();

        loop {
            p = Vec3::new(
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector_lambertian() -> Vec3 {
        let mut rng = rand::thread_rng();

        let a = rng.gen::<f64>() * PI; //random value between 0.0 and PI
        let z = rng.gen::<f64>() * 2.0 - 1.0; //random value between -1.0 and 1.0
        let r = (1.0 - z * z).sqrt();

        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * *normal //&self is a reference to the incident ray, self dot normal scales the normal with the component
                                                 //of the incident ray in the direction of the normal (since the the normal is a unit vector)
    }

    pub fn refract(&self, normal: &Vec3, eta_over_etaprime: f64) -> Vec3 {
        let cos_theta = -self.dot(normal);

        //split the refracted ray into to components, one Parallel to the normal and the other perpendicular
        let ray_out_parallel = eta_over_etaprime * (*self + cos_theta * *normal);
        let ray_out_perpendicular = -(1.0 - ray_out_parallel.length_squared()).sqrt() * *normal;
        //Add them back together to form the refracted ray
        ray_out_parallel + ray_out_perpendicular
    }
}

#[allow(dead_code)]
impl Vec3Traits for Vec3 {
    type Kind = Vec3;

    fn first_element(&self) -> f64 {
        self.x
    }
    fn second_element(&self) -> f64 {
        self.y
    }
    fn third_element(&self) -> f64 {
        self.z
    }

    fn cross(&self, rhs: &Self::Kind) -> Self::Kind {
        Self::Kind {
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.x - self.x * rhs.z),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }

    fn unit_vector(&self) -> Self::Kind {
        *self / self.length()
    }
}

//overloading opertors
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (rhs.x * self),
            y: (rhs.y * self),
            z: (rhs.z * self),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::vec3::*;

    #[test]
    fn test_vec3_cross() {
        assert_eq!(
            Vec3::new(0.0, 1.0, 0.0).cross(&Vec3::new(-2.0, 2.0, 0.0)),
            Vec3::new(0.0, 0.0, 2.0)
        );
    }

    #[test]
    fn test_vec3_dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(&Vec3::new(1.0, 5.0, 7.0)) as usize,
            32.0 as usize
        );
    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_div() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(3.0, 2.0, 1.0),
            Vec3::new(4.0, 4.0, 4.0)
        );
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(3.0, 2.0, 1.0),
            Vec3::new(-2.0, 0.0, 2.0)
        );
    }

    #[test]
    fn test_vec3_length() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).length() as usize,
            3.741_657_386_773_941_3 as usize
        );
    }

    #[test]
    fn test_vec3_unit_vector() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).unit_vector(),
            Vec3::new(
                0.267_261_241_912_424_4,
                0.534_522_483_824_848_8,
                0.801_783_725_737_273_2
            )
        );
    }
}
