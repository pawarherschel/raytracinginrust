#![allow(unused)]
#![feature(trace_macros)]
#![feature(portable_simd)]
#![feature(slice_first_last_chunk)]

#[cfg(test)]
#[macro_use]
pub use rstest_reuse;
#[macro_use]
pub use rstest_reuse::*;

pub mod camera;
pub mod config;
// pub mod cube;
pub mod cimpl;
pub mod color_macros;
pub mod hit;
pub mod macros;
pub mod material;
pub mod math;
pub mod point3_macros;
pub mod prelude;
pub mod ray;
pub mod sphere;
mod test_cases;
pub mod util;
pub mod vec3;
pub mod vec3_macros;
pub mod world;
