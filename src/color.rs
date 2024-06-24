use std::f64::INFINITY;

use crate::{
    hittable::{Hittable, HittableList},
    ray::Ray,
    vec3::Vector3,
};

pub fn write_color(pixel_color: Vector3) {
    let ir = (255.999 * pixel_color[0]) as u64;
    let ig = (255.999 * pixel_color[1]) as u64;
    let ib = (255.999 * pixel_color[2]) as u64;

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vector3 {
    if let Some(hit_record) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (hit_record.normal + Vector3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);
    return (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
}
