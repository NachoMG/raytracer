use std::f64::INFINITY;

use crate::{
    hittable::{Hittable, HittableList},
    vec3::Vector3,
};

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, hittable_list: &HittableList) -> Vector3 {
        if let Some(hit_record) = hittable_list.hit(self, 0.0, INFINITY) {
            return 0.5 * (hit_record.normal + Vector3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction[1] + 1.0);
        return (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
    }
}
