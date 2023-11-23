#![allow(unused)]

mod math;
mod ray;
mod vec3;

pub type Vec3 = vec3::Vec3;
#[macro_export]
macro_rules! vec3 {
    ($l: expr) => {{
        Vec3::new($l as f64, $l as f64, $l as f64)
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Vec3::new($l0 as f64, $l1 as f64, $l2 as f64)
    }};
}

pub type Color = Vec3;
#[macro_export]
macro_rules! color {
    ($l: expr) => {{
        Color::new($l as f64, $l as f64, $l as f64)
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Color::new($l0 as f64, $l1 as f64, $l2 as f64)
    }};
}

pub type Point3 = Vec3;
#[macro_export]
macro_rules! point3 {
    ($l: expr) => {{
        Point3::new($l as f64, $l as f64, $l as f64)
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Point3::new($l0 as f64, $l1 as f64, $l2 as f64)
    }};
}

pub type Ray = ray::Ray;
