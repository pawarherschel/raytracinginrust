use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Range, Sub};
use std::simd::f32x4;

use rand::prelude::*;

use crate::config::{NEAR_ZERO_EPSILON, SAMPLES_PER_PIXEL};
use crate::prelude::Color;
use crate::vec3;

#[derive(Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    #[inline(always)]
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self([a, b, c])
    }
    #[inline(always)]
    pub fn new_from_array(array: [f32; 3]) -> Self {
        Self(array)
    }
    #[inline(always)]
    pub fn new_from_f32x4(simd: f32x4) -> Self {
        // let op = *simd.as_array().first_chunk().unwrap();
        // let array = simd.to_array();
        Self(*simd.as_array().first_chunk().unwrap())
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn x(&self) -> f32 {
        self[0]
    }
    #[inline(always)]
    pub fn y(&self) -> f32 {
        self[1]
    }
    #[inline(always)]
    pub fn z(&self) -> f32 {
        self[2]
    }

    #[inline(always)]
    pub fn r(&self) -> f32 {
        self[0]
    }
    #[inline(always)]
    pub fn g(&self) -> f32 {
        self[1]
    }
    #[inline(always)]
    pub fn b(&self) -> f32 {
        self[2]
    }

    #[inline(always)]
    pub fn get_f32x4(&self) -> f32x4 {
        f32x4::from_array([self.0[0], self.0[1], self.0[2], 0.0])
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3 {
        self.clone() / self.length()
    }

    #[inline(always)]
    pub fn is_near_zero(&self) -> bool {
        self.abs() < NEAR_ZERO_EPSILON.abs()
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Self) -> Self {
        self - &(2.0 * self.dot(normal) * normal)
    }
}

impl Vec3 {
    // todo!("Optimize this")
    #[inline(always)]
    pub fn random(r: Range<f32>) -> Vec3 {
        let mut rng = thread_rng();

        vec3![rng.gen_range(r.clone())]
    }

    // todo!("Optimize this")
    #[inline(always)]
    pub fn random_in_unit_sphere() -> Vec3 {
        let v = Vec3::random(-1.0..1.0);

        if v.length() < 1.0 {
            v
        } else {
            v.normalize()
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl PartialEq<f32> for Vec3 {
    #[inline(always)]
    fn eq(&self, other: &f32) -> bool {
        self[0] == *other && self[1] == *other && self[2] == *other
    }
}

impl PartialOrd<f32> for Vec3 {
    #[inline(always)]
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
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
    #[inline(always)]
    pub fn fmt_color(&self) -> String {
        let Color([r, g, b]) =
            (256_f32 * &((((self) / SAMPLES_PER_PIXEL as f32).sqrt()).clamp(0.0, 0.999)));
        let (r, g, b) = (r as u64, g as u64, b as u64);
        format!("{} {} {}", r, g, b)
    }

    #[inline(always)]
    pub fn fmt_u8(&self) -> [u8; 3] {
        let Color([r, g, b]) =
            (256_f32 * &((((self) / SAMPLES_PER_PIXEL as f32).sqrt()).clamp(0.0, 0.999)));
        let (r, g, b) = (r as u8, g as u8, b as u8);
        [r, g, b]
    }
}
//
// impl Sub<&Vec3> for &Vec3 {
//     type Output = Vec3;
//
//     #[inline(always)]
//     fn sub(self, rhs: &Color) -> Self::Output {
//         let left = self.get_f32x4();
//         let right = rhs.get_f32x4();
//
//         Self::Output::new_from_f32x4(left - right)
//     }
// }
