use crate::Point3;

#[derive(Clone, Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &self.direction * t
    }

    pub fn get_origin(&self) -> Point3 {
        self.origin.clone()
    }

    pub fn get_direction(&self) -> Point3 {
        self.direction.clone().normalize()
    }

    pub fn get_direction_denormalized(&self) -> Point3 {
        self.direction.clone()
    }
}
