use rand::Rng;

use crate::{
    color::{ray_color, write_color},
    hittable::HittableList,
    ray::Ray,
    utils::degrees_to_radians,
    vec3::Vector3,
};

pub struct Camera {
    image_width: i32,         // Rendered image width in pixel count
    image_height: i32,        // Rendered image height in pixel count
    center: Vector3,          //
    pixel00_loc: Vector3,     //
    pixel_delta_u: Vector3,   //
    pixel_delta_v: Vector3,   //
    samples_per_pixel: i32,   // Count of random samples for each pixel
    pixel_samples_scale: f64, //
    max_depth: i32,           // Maximum number of ray bounces into scene
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        v_fov: f64,
    ) -> Camera {
        let mut image_height = ((image_width as f64) / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);

        let center = Vector3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    fn sample_square() -> Vector3 {
        let mut rnd = rand::thread_rng();
        Vector3::new(rnd.gen::<f64>() - 0.5, rnd.gen::<f64>() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset[0]) * self.pixel_delta_u)
            + (((j as f64) + offset[0]) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(&self, world: HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j - 1);

            for i in 0..self.image_width {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray_color(&ray, self.max_depth, &world);
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }

        eprintln!("\nDone.");
    }
}
