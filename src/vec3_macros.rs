use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::{Add, Neg};

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
macro_rules! cimpl {
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&mut self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&mut self, rhs: $rhs) {
                self[0] $op rhs[0];
                self[1] $op rhs[1];
                self[2] $op rhs[2];
            }
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(self) -> Self::Output {
                Self::Output::new(
                    $op self[0],
                    $op self[1],
                    $op self[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {

        impl $trait for $lhs {
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    // reflect
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&mut self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(self $op rhs[0], self $op rhs[1], self $op rhs[2])
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $fn(&mut self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;

            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            pub fn $fn(self) -> $output {
                <$output>::new(
                    self[0].$action(),
                    self[1].$action(),
                    self[2].$action()
                )
            }
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    // cimpl!(self: Vec3, output: Vec3, fn: clamp, perform: clamp, args: min:f64, max:f64 | 3);
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            pub fn $fn(self, $($arg_ident:$arg_ty, )+) -> $output {
                <$output>::new(
                    self[0].$action($($arg_ident, )+),
                    self[1].$action($($arg_ident, )+),
                    self[2].$action($($arg_ident, )+)
                )
            }
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            pub fn $fn(&self, $($arg_ident:$arg_ty)+) -> $output  {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            pub fn $fn(&mut self, $($arg_ident:$arg_ty)+) -> $output  {unimplemented!()}
        }
    };
}

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

// Vec3 w/ f64
cimpl!(trait: Add, self: Vec3, other: f64, output: Vec3, fn: add, op: + | 3, 1);
cimpl!(trait: Sub, self: Vec3, other: f64, output: Vec3, fn: sub, op: - | 3, 1);
cimpl!(trait: Mul, self: Vec3, other: f64, output: Vec3, fn: mul, op: * | 3, 1);
cimpl!(trait: Mul, self: &Vec3, other: f64, output: Vec3, fn: mul, op: * | 3, 1);
cimpl!(trait: Div, self: Vec3, other: f64, output: Vec3, fn: div, op: / | 3, 1);
cimpl!(trait: Div, self: &Vec3, other: f64, output: Vec3, fn: div, op: / | 3, 1);

// f64 w/ Vec3
cimpl!(trait: Add, self: f64, other: Vec3, output: Vec3, fn: add, op: + | 1, 3);
cimpl!(trait: Mul, self: f64, other: Vec3, output: Vec3, fn: mul, op: * | 1, 3);
cimpl!(trait: Mul, self: f64, other: &Vec3, output: Vec3, fn: mul, op: * | 1, 3);

// unary Vec3
cimpl!(trait: Neg, self: Vec3, output: Vec3, fn: neg, op: - | 3);
cimpl!(self: Vec3, output: Vec3, fn: sqrt, perform: sqrt | 3);
cimpl!(self: Vec3, output: Vec3, fn: clamp, perform: clamp, args: min:f64, max:f64, | 3);
