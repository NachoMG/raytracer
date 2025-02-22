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
    defocus_angle: f64,       // Variation angle of rays through each pixel
    defocus_disk_u: Vector3,  // Defocus disk horizontal radius
    defocus_disk_v: Vector3,  // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        v_fov: f64,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let mut image_height = ((image_width as f64) / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);

        let center = look_from;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn sample_square() -> Vector3 {
        let mut rnd = rand::thread_rng();
        Vector3::new(rnd.gen::<f64>() - 0.5, rnd.gen::<f64>() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset[0]) * self.pixel_delta_u)
            + (((j as f64) + offset[0]) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn defocus_disk_sample(&self) -> Vector3 {
        let p = Vector3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
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
