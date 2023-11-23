use crate::{point3, x, y, z, Point3};

// IMAGE
pub static ASPECT_RATIO: f64 = 16.0 / 9.0;
// pub static IMAGE_WIDTH: u64 = 256;
pub static IMAGE_WIDTH: u64 = 256;
pub static IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
pub static SAMPLES_PER_PIXEL: u64 = 100;

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
