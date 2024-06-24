use crate::{
    color::{ray_color, write_color},
    hittable::HittableList,
    ray::Ray,
    vec3::Vector3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64) -> Camera {
        let mut image_height = ((image_width as f64) / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let center = Vector3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
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
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j - 1);

            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + ((i as f64) * self.pixel_delta_u)
                    + ((j as f64) * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                write_color(ray_color(&ray, &world));
            }
        }

        eprintln!("\nDone.");
    }
}
