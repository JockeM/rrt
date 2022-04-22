pub type Vec3 = cgmath::Vector3<f64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: [u8; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub origin: Vec3,
    pub color: [u8; 4],
}

impl Sphere {
    pub fn get_normal(&self, point: &Vec3) -> Vec3 {
        return (point - self.origin) * (-1.0 / (self.radius));
    }
}
