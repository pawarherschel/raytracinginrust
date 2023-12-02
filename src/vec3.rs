use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
};

use rand::prelude::*;

use crate::config::{NEAR_ZERO_EPSILON, SAMPLES_PER_PIXEL};
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

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

macro_rules! cimpl {
    // A + B
    (trait: $trait:ty, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(self, rhs: $rhs) -> Self::Output $body
        }
    };
    (trait: $trait:ty, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&self, rhs: $rhs) -> Self::Output $body
        }
    };
    (trait: $trait:ty, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&mut self, rhs: $rhs) -> Self::Output $body
        }
    };
    // A += B
    (trait: $trait:ty, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(self, rhs: $rhs) $body
        }
    };
    (trait: $trait:ty, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&self, rhs: $rhs) $body
        }
    };
    (trait: $trait:ty, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&mut self, rhs: $rhs) $body
        }
    };
    // -A
    (trait: $trait:ty, self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(self) -> Self::Output $body
        }
    };
    (trait: $trait:ty, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(&self) -> Self::Output $body
        }
    };
    (trait: $trait:ty, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(&mut self) -> Self::Output $body
        }
    };
    // IDK
    (trait: $trait:ty, self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            fn $fn(self) $body
        }
    };
    (trait: $trait:ty, ref self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            fn $fn(&self) $body
        }
    };
    (trait: $trait:ty, ref mut self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $trait for $lhs {
            fn $fn(&mut self) $body
        }
    };
    // reflect
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(self, rhs: $rhs) -> $output $body
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&self, rhs: $rhs) -> $output $body
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&mut self, rhs: $rhs) -> $output $body
        }
    };
    // IDK
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(self, rhs: $rhs) $body
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&self, rhs: $rhs) $body
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&mut self, rhs: $rhs) $body
        }
    };
    // IDK
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(self) -> $output $body
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&self) -> $output $body
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&mut self) -> $output $body
        }
    };
    // IDK
    (self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(self) $body
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&self) $body
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, body: $body:block) => {
        impl $lhs {
            fn $fn(&mut self) $body
        }
    };
}

trace_macros!(true);
cimpl!(trait: std::ops::AddAssign, ref mut self: crate::vec3::Vec3, other: crate::vec3::Vec3, fn: add_assign, body: {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
});
trace_macros!(false);
//
// impl AddAssign<Vec3> for Vec3 {
//     fn add_assign(&mut self, rhs: Vec3) {
//         self[0] += rhs[0];
//         self[1] += rhs[1];
//         self[2] += rhs[2];
//     }
// }

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl AddAssign<Vec3> for &mut Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] + rhs, self[1] + rhs, self[2] + rhs)
    }
}

impl Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] + rhs, self[1] + rhs, self[2] + rhs)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs[0] + self, rhs[1] + self, rhs[2] + self)
    }
}

impl Add<&Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs[0] + self, rhs[1] + self, rhs[2] + self)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl SubAssign<Vec3> for &mut Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] - rhs, self[1] - rhs, self[2] - rhs)
    }
}

impl Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] - rhs, self[1] - rhs, self[2] - rhs)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs[0] - self, rhs[1] - self, rhs[2] - self)
    }
}

impl Sub<&Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs[0] - self, rhs[1] - self, rhs[2] - self)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl DivAssign<f64> for &mut Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
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
