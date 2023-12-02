use std::sync::Arc;

use crate::hit::{HitRecord, Hittable};
use crate::material::Scatter;
use crate::prelude::*;
use crate::value;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

unsafe impl Send for Sphere {}

unsafe impl Sync for Sphere {}

impl Sphere {
    #[inline(always)]
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let r_direction = ray.get_direction_denormalized();

        /// float sdSphere( vec3 p, float s )
        /// {
        ///     return length(p)-s;
        /// }
        let a = r_direction.length().powi(2);
        let half_b = oc.dot(&r_direction);
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let root_pos = (-half_b + discriminant_sqrt) / a;
        let root_neg = (-half_b - discriminant_sqrt) / a;

        let root = match (
            value!(root_pos, between: t_min, and t_max),
            value!(root_neg, between: t_min, and t_max),
        ) {
            (false, false) => return None,
            (false, true) => root_neg,
            (true, false) => root_pos,
            (true, true) => root_neg,
        };

        let point = ray.at(root);
        let outward_normal = (&point - &self.center) / self.radius;
        let t = root;

        Some(HitRecord::new(point, t, self.material.clone()).set_front_face(ray, outward_normal))
    }
}
