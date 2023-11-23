#![allow(unused)]

pub mod hit;
pub mod math;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod world;

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

#[macro_export]
macro_rules! red {
    ($red: expr) => {{
        color![1, 0, 0] * $red as f64
    }};
    () => {{
        red!(1)
    }};
}

#[macro_export]
macro_rules! green {
    ($green: expr) => {{
        color![0, 1, 0] * $green as f64
    }};
    () => {{
        green!(1)
    }};
}

#[macro_export]
macro_rules! blue {
    ($blue: expr) => {{
        color![0, 0, 1] * $blue as f64
    }};
    () => {{
        blue!(1)
    }};
}

#[macro_export]
macro_rules! white {
    () => {{
        color!(1)
    }};
}

pub type Point3 = Vec3;
#[macro_export]
macro_rules! point3 {
    ($l: expr) => {{
        Point3 {
            0: [$l as f64, $l as f64, $l as f64],
        }
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Point3 {
            0: [$l0 as f64, $l1 as f64, $l2 as f64],
        }
    }};
}

#[macro_export]
macro_rules! x {
    ($x: expr) => {{
        point3![1_f64 * $x as f64, 0, 0]
    }};
    () => {{
        x!(1)
    }};
}

#[macro_export]
macro_rules! y {
    ($y: expr) => {{
        point3![0, 1_f64 * $y as f64, 0]
    }};
    () => {{
        y!(1)
    }};
}

#[macro_export]
macro_rules! z {
    ($z: expr) => {{
        point3![0, 0, -1_f64 * $z as f64]
    }};
    () => {{
        z!(1)
    }};
}

pub type Ray = ray::Ray;
