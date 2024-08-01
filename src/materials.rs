use rand::Rng;

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
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        let albedo = Vector3::new(1.0, 1.0, 1.0);
        let refraction_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_index)
                > rand::thread_rng().gen_range(0.0..1.0)
        {
            Vector3::reflect(unit_direction, hit_record.normal)
        } else {
            Vector3::refract(unit_direction, hit_record.normal, refraction_index)
        };

        let scattered_ray = Ray::new(hit_record.p, direction);

        Some((albedo, scattered_ray))
    }
}
