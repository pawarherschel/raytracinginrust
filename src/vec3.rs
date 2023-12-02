use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Range, Sub};

use rand::prelude::*;

use crate::config::{NEAR_ZERO_EPSILON, SAMPLES_PER_PIXEL};
use crate::prelude::Color;
use crate::vec3;

#[derive(Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self([a, b, c])
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn r(&self) -> f64 {
        self[0]
    }
    pub fn g(&self) -> f64 {
        self[1]
    }
    pub fn b(&self) -> f64 {
        self[2]
    }
}

impl Vec3 {
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn abs(&self) -> Self {
        vec3![self[0].abs(), self[1].abs(), self[2].abs()]
    }

    pub fn is_near_zero(&self) -> bool {
        self.abs() < NEAR_ZERO_EPSILON.abs()
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl Vec3 {
    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = thread_rng();

        vec3![rng.gen_range(r.clone())]
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);

            if v.length() < 1.0 {
                return v;
            }
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl PartialEq<f64> for Vec3 {
    fn eq(&self, other: &f64) -> bool {
        self[0] == *other && self[1] == *other && self[2] == *other
    }
}

impl PartialOrd<f64> for Vec3 {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        let o0 = self[0].partial_cmp(other).unwrap();
        let o1 = self[1].partial_cmp(other).unwrap();
        let o2 = self[2].partial_cmp(other).unwrap();

        if o0 == o1 && o1 == o2 {
            Some(o0)
        } else {
            None
        }
    }
}

impl Vec3 {
    pub fn fmt_color(&self) -> String {
        let Color([r, g, b]) =
            (256_f64 * ((((self) / SAMPLES_PER_PIXEL as f64).sqrt()).clamp(0.0, 0.999)));
        let (r, g, b) = (r as u64, g as u64, b as u64);
        format!("{} {} {}", r, g, b)
    }
}
