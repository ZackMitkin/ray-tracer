use crate::utils::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray { direction, origin }
    }
    pub fn set(&mut self, origin: Vec3, direction: Vec3) {
        *self = Ray { direction, origin }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
}
