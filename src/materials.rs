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

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        let reflection = Vector3::reflect(r_in.direction, hit_record.normal)
            + (self.fuzz * Vector3::random_unit_vector());
        let scattered_ray = Ray::new(hit_record.p, reflection);

        if scattered_ray.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}
