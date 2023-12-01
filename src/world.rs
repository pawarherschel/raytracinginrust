use std::sync::Arc;

use crate::hit::{HitRecord, Hittable};
use crate::*;

pub type World = Arc<Vec<Arc<dyn Hittable + Send + Sync>>>;

impl Hittable for World {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // iterate through all hittable objects and if they intersect then get
        // the closest to camera
        let (output, _) =
            self.clone()
                .iter()
                .fold((None, t_max), |(temp_record, closest_so_far), hittable| {
                    if let Some(record) = hittable.hit(ray, t_min, closest_so_far) {
                        (Some(record.clone()), record.t)
                    } else {
                        (temp_record, closest_so_far)
                    }
                });

        output
    }
}
