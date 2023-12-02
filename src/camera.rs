use crate::config::{HORIZONTAL, LOWER_LEFT_CORNER, VERTICAL};
use crate::prelude::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    #[inline(always)]
    fn default() -> Self {
        Camera {
            origin: ORIGIN.clone(),
            horizontal: HORIZONTAL.clone(),
            lower_left_corner: LOWER_LEFT_CORNER.clone(),
            vertical: VERTICAL.clone(),
        }
    }
}

impl Camera {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + u * self.horizontal.clone()
                + v * self.vertical.clone()
                - self.origin.clone(),
        )
    }
}
