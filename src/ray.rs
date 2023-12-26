use crate::hit::Hittable;
use crate::prelude::*;
use crate::{color, lerp, remap, white};

#[derive(Clone, Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    #[inline(always)]
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }

    #[inline(always)]
    pub fn get_origin(&self) -> Point3 {
        self.origin.clone()
    }

    #[inline(always)]
    pub fn get_direction(&self) -> Point3 {
        self.direction.clone().normalize()
    }

    #[inline(always)]
    pub fn get_direction_denormalized(&self) -> Point3 {
        self.direction.clone()
    }

    #[inline(always)]
    pub fn color(&self, world: &World, depth: u64) -> Color {
        if depth == 0 {
            return color![0];
        }
        if let Some(record) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some((attenuation_color, scattered_ray)) = record.material.scatter(self, &record)
            {
                attenuation_color * scattered_ray.color(world, depth - 1)
            } else {
                color!(0)
            }
            // remap!(value: record.normal.unwrap(), from: -1_f64, 1_f64, to: 0_f64, 1_f64)
        } else {
            let direction = self.get_direction();
            let t = remap!(value: direction.y(), from: -1_f64, 1_f64, to: 0_f64, 1_f64);

            lerp!(&white!(), t, &color![0.5, 0.7, 1])
        }
    }
}
