use crate::utils::vec3::Vec3;

use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from(0.0, viewport_height, 0.0);
        let origin = Vec3::new();

        Camera {
            origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::from(0.0, 0.0, focal_length),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::from(self.origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
