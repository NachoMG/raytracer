use std::sync::Arc;

use camera::Camera;
use hittable::HittableList;
use materials::Lambertian;
use sphere::Sphere;
use vec3::Vector3;

mod camera;
mod color;
mod hittable;
mod materials;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));

    world.push(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        material_ground,
    ));
    world.push(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_center,
    ));

    let camera = Camera::new(400, 16.0 / 9.0, 100, 50);
    camera.render(world);
}
