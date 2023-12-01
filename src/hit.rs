use std::sync::Arc;

use crate::material::Scatter;
use crate::{Point3, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Option<Vec3>,
    pub material: Arc<dyn Scatter>,
    pub t: f64,
    pub front_face: Option<bool>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(point: Point3, t: f64, material: Arc<dyn Scatter>) -> Self {
        HitRecord {
            point,
            normal: None,
            material,
            t,
            front_face: None,
        }
    }

    pub fn set_front_face(self, ray: &Ray, outward_normal: Vec3) -> Self {
        let mut new = self;

        new.front_face = Some(ray.get_direction_denormalized().dot(&outward_normal) < 0.0);
        new.normal = Some(if new.front_face.unwrap() {
            outward_normal
        } else {
            -outward_normal
        });

        new
    }
}
