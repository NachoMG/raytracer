use crate::vec3::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }

    pub fn hit_sphere(&self, center: Vector3, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.direction.dot(self.direction);
        let b = -2.0 * self.direction.dot(oc);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }

    pub fn color(&self) -> Vector3 {
        let t = self.hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Vector3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Vector3::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0);
        }

        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction[1] + 1.0);
        return (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
    }
}
