use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::{Add, Neg};
use std::simd::f64x4;

use crate::cimpl;
use crate::vec3::Vec3;

#[macro_export]
macro_rules! vec3 {
    ($l: expr) => {{
        vec3!($l, $l, $l)
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Vec3([$l0 as f64, $l1 as f64, $l2 as f64])
    }};
}

#[macro_export]
macro_rules! impl_simd {
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$lhs> for $rhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                let left = self.get_f64x4();
                let right = rhs.get_f64x4();

                Self::Output::new_from_f64x4(left $op right)
            }
        }
    };
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                let left = self.get_f64x4();
                let right = f64x4::splat(rhs);

                Self::Output::new_from_f64x4(left $op right)
            }
        }
    };
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                let left = f64x4::splat(self);
                let right = rhs.get_f64x4();

                Self::Output::new_from_f64x4(left $op right)
            }
        }
    };
}

//
// Vec3 w/ Vec3
cimpl!(trait: Add, self: Vec3, other: Vec3, output: Vec3, fn: add, op: + | 3, 3);
cimpl!(trait: Add, self: Vec3, other: &Vec3, output: Vec3, fn: add, op: + | 3, 3);
cimpl!(trait: Add, self: &Vec3, other: Vec3, output: Vec3, fn: add, op: + | 3, 3);
cimpl!(trait: Add, self: &Vec3, other: &Vec3, output: Vec3, fn: add, op: + | 3, 3);

cimpl!(trait: Sub, self: Vec3, other: Vec3, output: Vec3, fn: sub, op: - | 3, 3);
cimpl!(trait: Sub, self: Vec3, other: &Vec3, output: Vec3, fn: sub, op: - | 3, 3);
cimpl!(trait: Sub, self: &Vec3, other: Vec3, output: Vec3, fn: sub, op: - | 3, 3);
cimpl!(trait: Sub, self: &Vec3, other: &Vec3, output: Vec3, fn: sub, op: - | 3, 3);

cimpl!(trait: Mul, self: Vec3, other: Vec3, output: Vec3, fn: mul, op: * | 3, 3);
cimpl!(trait: Mul, self: Vec3, other: &Vec3, output: Vec3, fn: mul, op: * | 3, 3);
cimpl!(trait: Mul, self: &Vec3, other: Vec3, output: Vec3, fn: mul, op: * | 3, 3);
cimpl!(trait: Mul, self: &Vec3, other: &Vec3, output: Vec3, fn: mul, op: * | 3, 3);

cimpl!(trait: AddAssign, ref mut self: Vec3, other: Vec3, fn: add_assign, op: += | 3, 3);
cimpl!(trait: AddAssign, ref mut self: Vec3, other: &Vec3, fn: add_assign, op: += | 3, 3);

cimpl!(trait: SubAssign, ref mut self: Vec3, other: Vec3, fn: sub_assign, op: -= | 3, 3);
cimpl!(trait: SubAssign, ref mut self: Vec3, other: &Vec3, fn: sub_assign, op: -= | 3, 3);

cimpl!(trait: MulAssign, ref mut self: Vec3, other: Vec3, fn: mul_assign, op: *= | 3, 3);
cimpl!(trait: MulAssign, ref mut self: Vec3, other: &Vec3, fn: mul_assign, op: *= | 3, 3);

//
// Vec3 w/ f64
cimpl!(trait: Add, self: Vec3, other: f64, output: Vec3, fn: add, op: + | 3, 1);

cimpl!(trait: Sub, self: Vec3, other: f64, output: Vec3, fn: sub, op: - | 3, 1);
cimpl!(trait: Sub, self: &Vec3, other: f64, output: Vec3, fn: sub, op: - | 3, 1);

cimpl!(trait: Mul, self: Vec3, other: f64, output: Vec3, fn: mul, op: * | 3, 1);
cimpl!(trait: Mul, self: &Vec3, other: f64, output: Vec3, fn: mul, op: * | 3, 1);

cimpl!(trait: Div, self: Vec3, other: f64, output: Vec3, fn: div, op: / | 3, 1);
cimpl!(trait: Div, self: &Vec3, other: f64, output: Vec3, fn: div, op: / | 3, 1);

//
// f64 w/ Vec3
cimpl!(trait: Add, self: f64, other: Vec3, output: Vec3, fn: add, op: + | 1, 3);
cimpl!(trait: Add, self: f64, other: &Vec3, output: Vec3, fn: add, op: + | 1, 3);
cimpl!(trait: Sub, self: f64, other: Vec3, output: Vec3, fn: sub, op: - | 1, 3);
cimpl!(trait: Sub, self: f64, other: &Vec3, output: Vec3, fn: sub, op: - | 1, 3);

// cimpl!(trait: Mul, self: f64, other: Vec3, output: Vec3, fn: mul, op: * | 1, 3);
cimpl!(trait: Mul, self: f64, other: &Vec3, output: Vec3, fn: mul, op: * | 1, 3);

//
// unary Vec3
cimpl!(trait: Neg, self: Vec3, output: Vec3, fn: neg, op: - | 3);

cimpl!(self: Vec3, output: Vec3, fn: sqrt, perform: sqrt | 3);
cimpl!(ref self: Vec3, output: Vec3, fn: abs, perform: abs | 3);

cimpl!(self: Vec3, output: Vec3, fn: clamp, perform: clamp, args: min:f64, max:f64, | 3);
