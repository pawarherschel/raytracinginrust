#![allow(unused)]

pub mod camera;
pub mod config;
// pub mod cube;
pub mod color;
pub mod hit;
pub mod macros;
pub mod material;
pub mod math;
pub mod point3;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;
pub mod vec3_macros;
pub mod world;

pub type Vec3 = vec3::IVec3;
pub type Color = vec3::IVec3;
pub type Point3 = vec3::IVec3;
pub type Ray = ray::IRay;
