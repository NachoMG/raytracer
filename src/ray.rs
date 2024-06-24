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
}
