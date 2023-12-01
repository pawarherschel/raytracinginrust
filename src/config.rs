use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::material::{Lambertian, Metal};
use crate::{color, point3, x, y, z, Color, Point3};

// IMAGE
pub static ASPECT_RATIO: f64 = 16.0 / 9.0;
// pub static HI_RES: bool = cfg!(debug_assertions); // for testing with AMD uPerf
pub static HI_RES: bool = cfg!(not(debug_assertions));
// normal operation
pub static IMAGE_WIDTH: u64 = if HI_RES { 1920 } else { 256 };
pub static IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
pub static SAMPLES_PER_PIXEL: u64 = 2 << 6;
pub static MAX_DEPTH: u64 = 5;

// CAMERA
pub static VIEWPORT_HEIGHT: f64 = 2.0;
pub static VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
pub static FOCAL_LENGTH: f64 = 1.0;
pub static ORIGIN: &Point3 = &point3![0];
pub static HORIZONTAL: &Point3 = &x!(VIEWPORT_WIDTH);
pub static VERTICAL: &Point3 = &y!(VIEWPORT_HEIGHT);
pub static LOWER_LEFT_CORNER: &Point3 = &point3![
    ORIGIN.0[0] - HORIZONTAL.0[0] / 2.0 - VERTICAL.0[0] / 2.0 + z!(FOCAL_LENGTH).0[0],
    ORIGIN.0[1] - HORIZONTAL.0[1] / 2.0 - VERTICAL.0[1] / 2.0 + z!(FOCAL_LENGTH).0[1],
    ORIGIN.0[2] - HORIZONTAL.0[2] / 2.0 - VERTICAL.0[2] / 2.0 + z!(FOCAL_LENGTH).0[2]
];
// &(ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 + z!(FOCAL_LENGTH));

// CONSTANTS
pub static NEAR_ZERO_EPSILON: f64 = 1.0e-8;

// MATERIALS
pub static GROUND_MATERIAL: Lazy<Arc<Lambertian>> =
    Lazy::new(|| Arc::new(Lambertian::new(color!(0.8, 0.8, 0.0))));
pub static CENTER_SPHERE_MATERIAL: Lazy<Arc<Lambertian>> =
    Lazy::new(|| Arc::new(Lambertian::new(color!(0.7, 0.3, 0.3))));
pub static LEFT_SPHERE_MATERIAL: Lazy<Arc<Metal>> =
    Lazy::new(|| Arc::new(Metal::new(color!(0.8), 0.3)));
pub static RIGHT_SPHERE_MATERIAL: Lazy<Arc<Metal>> =
    Lazy::new(|| Arc::new(Metal::new(color!(0.8, 0.6, 0.2), 1.0)));
