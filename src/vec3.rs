use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Range, Sub};

use rand::prelude::*;

use crate::config::{NEAR_ZERO_EPSILON, SAMPLES_PER_PIXEL};
use crate::vec3;

#[derive(Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    #[inline(always)]
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self([a, b, c])
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn x(&self) -> f64 {
        self[0]
    }
    #[inline(always)]
    pub fn y(&self) -> f64 {
        self[1]
    }
    #[inline(always)]
    pub fn z(&self) -> f64 {
        self[2]
    }

    #[inline(always)]
    pub fn r(&self) -> f64 {
        self[0]
    }
    #[inline(always)]
    pub fn g(&self) -> f64 {
        self[1]
    }
    #[inline(always)]
    pub fn b(&self) -> f64 {
        self[2]
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
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
    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3 {
        self.clone() / self.length()
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        vec3![self[0].abs(), self[1].abs(), self[2].abs()]
    }

    #[inline(always)]
    pub fn is_near_zero(&self) -> bool {
        self.abs() < NEAR_ZERO_EPSILON.abs()
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = thread_rng();

        vec3![rng.gen_range(r.clone())]
    }

    #[inline(always)]
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

impl PartialEq<f64> for Vec3 {
    fn eq(&self, other: &f64) -> bool {
        self[0] == *other && self[1] == *other && self[2] == *other
    }
}

impl PartialOrd<f64> for Vec3 {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        let o0 = self[0].partial_cmp(other);
        let o1 = self[1].partial_cmp(other);
        let o2 = self[2].partial_cmp(other);

        if o0 == o1 && o1 == o2 {
            o0
        } else {
            None
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Vec3 {
    pub fn fmt_color(&self) -> String {
        let r = (256_f64
            * (self.r() / SAMPLES_PER_PIXEL as f64)
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let g = (256_f64
            * (self.g() / SAMPLES_PER_PIXEL as f64)
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let b = (256_f64
            * (self.b() / SAMPLES_PER_PIXEL as f64)
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        format!("{} {} {}", r, g, b)
    }
}

macro_rules! pairwise_operation_generic {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
}

macro_rules! pairwise_operation {
    ($trait:ident, $fn_name:ident, $op:tt) => {
        pairwise_operation_generic!(Vec3, Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(Vec3, &Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(&Vec3, Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(&Vec3, &Vec3, Vec3, $trait, $fn_name, $op);
    };
}

macro_rules! operation_generic {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
    ($lhs:ty, &$rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<&$rhs> for $lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: &$rhs) -> Self::Output {
                Vec3::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
    (&$lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for &$lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
    (&$lhs:ty, &$rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<&$rhs> for &$lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: &$rhs) -> Self::Output {
                Vec3::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
}
macro_rules! operation_generic_flip {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self $op rhs[0],
                    self $op rhs[1],
                    self $op rhs[2],
                )
            }
        }
    };
    ($lhs:ty, &$rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<&$rhs> for $lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: &$rhs) -> Self::Output {
                Vec3::new(
                    self $op rhs[0],
                    self $op rhs[1],
                    self $op rhs[2],
                )
            }
        }
    };
    (&$lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for &$lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self $op rhs[0],
                    self $op rhs[1],
                    self $op rhs[2],
                )
            }
        }
    };
    (&$lhs:ty, &$rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<&$rhs> for &$lhs {
            type Output = $output_type;

            fn $fn_name(self, rhs: &$rhs) -> Self::Output {
                Vec3::new(
                    self $op rhs[0],
                    self $op rhs[1],
                    self $op rhs[2],
                )
            }
        }
    };
}

macro_rules! floating_point_operation {
    ($trait:ident, $fn_name:ident, $op:tt) => {
        operation_generic!(Vec3, f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(Vec3, &f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(&Vec3, f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(&Vec3, &f64, Vec3, $trait, $fn_name, $op);

        operation_generic_flip!(f64, Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(f64, &Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(&f64, Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(&f64, &Vec3, Vec3, $trait, $fn_name, $op);
    };
}

macro_rules! on_each_operation {
    ($trait:ident, $fn_name:ident) => {
        impl $trait for Vec3 {
            type Output = Vec3;

            fn $fn_name(self) -> Self::Output {
                Vec3::new(self[0].$fn_name(), self[1].$fn_name(), self[2].$fn_name())
            }
        }
    };
}

pairwise_operation!(Add, add, +);
pairwise_operation!(Sub, sub, -);

floating_point_operation!(Add, add, +);
floating_point_operation!(Sub, sub, -);
floating_point_operation!(Mul, mul, *);
floating_point_operation!(Div, div, /);

on_each_operation!(Neg, neg);
