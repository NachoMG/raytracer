use crate::{hittable::HitRecord, ray::Ray, vec3::Vector3};

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)>;
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.p, scatter_direction);
        Some((self.albedo, scattered_ray))
    }
}
