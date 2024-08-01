use std::f64::INFINITY;

use crate::{
    hittable::{Hittable, HittableList},
    ray::Ray,
    vec3::Vector3,
};

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel_color: Vector3) {
    let r = linear_to_gamma(pixel_color[0]);
    let g = linear_to_gamma(pixel_color[1]);
    let b = linear_to_gamma(pixel_color[2]);

    let ir = (256 as f64 * r.clamp(0.0, 0.999)) as u64;
    let ig = (256 as f64 * g.clamp(0.0, 0.999)) as u64;
    let ib = (256 as f64 * b.clamp(0.0, 0.999)) as u64;

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Vector3 {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        return match hit_record.material.scatter(ray, &hit_record) {
            Some((attenuation, scattered)) => attenuation * ray_color(&scattered, depth - 1, world),
            None => Vector3::new(0.0, 0.0, 0.0),
        };
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);

    (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
}
