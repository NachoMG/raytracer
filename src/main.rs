use std::sync::Arc;

use camera::Camera;
use hittable::HittableList;
use materials::{Dielectric, Lambertian, Metal};
use rand::Rng;
use sphere::Sphere;
use vec3::Vector3;

mod camera;
mod color;
mod hittable;
mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();
    let mut a = -11.0;
    while a < 11.0 {
        let mut b = -11.0;
        while b < 11.0 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vector3::random(0.0, 1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Vector3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
            };

            b += 1.0;
        }
        a += 1.0;
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));

    let camera = Camera::new(
        400,
        16.0 / 9.0,
        100,
        50,
        20.0,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    camera.render(world);
}
