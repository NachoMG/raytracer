use crate::{ray::Ray, vec3::Vector3};

pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vector3, normal: Vector3, t: f64) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vector3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
