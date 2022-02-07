use crate::{tracer::ray::Ray, utils::vec3::Vec3};

use super::material::Material;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &mut Ray,
        hit_record: &mut crate::objects::hit_record::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }

        scattered.set(hit_record.p, scatter_direction);
        attenuation.set(self.albedo);

        true
    }
}
