use crate::hit::{HitRecord, Hittable};
use crate::Ray;

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let mut temp_record = None;
        // let mut closest_so_far = t_max;
        //
        // for object in self {
        //     if let Some(record) = object.hit(ray, t_min, closest_so_far) {
        //         closest_so_far = record.t;
        //         temp_record = Some(record);
        //     }
        // }

        let (output, _) =
            self.iter()
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
