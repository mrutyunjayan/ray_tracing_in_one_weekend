use super::vec3::*;
use std::ops;

use rand::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}
#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }
    pub fn g(&self) -> f64 {
        self.g
    }
    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            r: rng.gen::<f64>(),
            g: rng.gen::<f64>(),
            b: rng.gen::<f64>(),
        }
    }

    //Write the tranlated [0,255] value of each color component
    /*
    pub fn write_color(pixel_color: &Color) {
        println!(
            "{} {} {}\n",
            (255.99 * pixel_color.r) as usize,
            (255.99 * pixel_color.g) as usize,
            (255.99 * pixel_color.b) as usize
        )
    }
    */
}

#[allow(dead_code)]
impl Vec3Traits for Color {
    type Kind = Color;

    fn first_element(&self) -> f64 {
        self.r
    }
    fn second_element(&self) -> f64 {
        self.g
    }
    fn third_element(&self) -> f64 {
        self.b
    }

    fn cross(&self, rhs: &Self) -> Self::Kind {
        Self::Kind {
            r: (self.g * rhs.b - self.b * rhs.g),
            g: (self.b * rhs.r - self.r * rhs.b),
            b: (self.r * rhs.g - self.g * rhs.r),
        }
    }

    fn unit_vector(&self) -> Self::Kind {
        self / self.length_squared()
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: (self.r + rhs.r),
            g: (self.g + rhs.g),
            b: (self.b + rhs.b),
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: (self.r - rhs.r),
            g: (self.g - rhs.g),
            b: (self.b - rhs.b),
        }
    }
}

impl ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: (self.r / rhs),
            g: (self.g / rhs),
            b: (self.b / rhs),
        }
    }
}

impl ops::Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: (self.r * rhs),
            g: (self.g * rhs),
            b: (self.b * rhs),
        }
    }
}
impl ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            r: (self * rhs.r),
            g: (self * rhs.g),
            b: (self * rhs.b),
        }
    }
}
