use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Vector3;

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();
    world.push(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(800, 16.0 / 9.0, 100);
    camera.render(world);
}
