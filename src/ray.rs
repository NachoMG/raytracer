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

    pub fn hit_sphere(&self, center: Vector3, radius: f64) -> bool {
        let oc = center - self.origin;
        let a = self.direction.dot(self.direction);
        let b = -2.0 * self.direction.dot(oc);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    pub fn color(&self) -> Vector3 {
        if self.hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5) {
            return Vector3::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}
