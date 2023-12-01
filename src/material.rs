use std::fmt::Debug;

use crate::hit::HitRecord;
use crate::{Color, Ray, Vec3};

pub trait Scatter: Debug {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction =
            hit_record.normal.clone().unwrap() + Vec3::random_in_unit_sphere().normalize();
        let scatter_direction = if scatter_direction.is_near_zero() {
            hit_record.normal.clone().unwrap()
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit_record.point.clone(), scatter_direction);

        Some((self.albedo.clone(), scattered))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in
            .direction
            .reflect(&hit_record.normal.clone().unwrap())
            .normalize();
        let scattered = Ray::new(hit_record.point.clone(), reflected);

        if scattered.direction.dot(&hit_record.normal.clone().unwrap()) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}
