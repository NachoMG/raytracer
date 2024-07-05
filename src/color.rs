use std::f64::INFINITY;

use crate::{
    hittable::{Hittable, HittableList},
    ray::Ray,
    vec3::Vector3,
};

pub fn write_color(pixel_color: Vector3) {
    let ir = (256 as f64 * pixel_color[0].clamp(0.0, 0.999)) as u64;
    let ig = (256 as f64 * pixel_color[1].clamp(0.0, 0.999)) as u64;
    let ib = (256 as f64 * pixel_color[2].clamp(0.0, 0.999)) as u64;

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Vector3 {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.0, INFINITY) {
        let direction = Vector3::random_on_hemisphere(hit_record.normal);
        return 0.5 * ray_color(&Ray::new(hit_record.p, direction), depth - 1, world);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);
    return (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
}
