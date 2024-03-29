//should this be sometype of global alias
type Vec3 = cgmath::Vector3<f64>;

use crate::objects::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn get_hitpoint(self, time: f64) -> Vec3 {
        return self.origin + (time * self.direction);
    }

    pub fn intersects_sphere(self, sphere: Sphere) -> Option<f64> {
        use crate::cgmath::InnerSpace;

        let delta = self.origin - sphere.origin;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * delta.dot(self.direction);
        let c = delta.dot(delta) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let numerator = -b - discriminant.sqrt();
            if numerator > 0.0 {
                return Some(numerator / (2.0 * a));
            }
        }
        let numerator = -b + discriminant.sqrt();
        if numerator > 0.0 {
            return Some(numerator / (2.0 * a));
        }
        return None;
    }
}
