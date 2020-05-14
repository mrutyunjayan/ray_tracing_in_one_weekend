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

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
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
    fn test_vec3_dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).cross(&Vec3::new(1.0, 5.0, 7.0)),
            Vec3::new(-1.0, -4.0, 3.0)
        );
    }

    #[test]
    fn test_vec3_cross() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(&Vec3::new(1.0, 5.0, 7.0)),
            32.0
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
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length(), 3.7416573867739413);
    }

    #[test]
    fn test_vec3_unit_vector() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).unit_vector(),
            Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
        );
    }
}
