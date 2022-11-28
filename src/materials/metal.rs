use crate::{
    tracer::ray::Ray,
    utils::vec3::{self, Vec3},
};

use super::material::Material;

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &mut Ray,
        hit_record: &mut crate::objects::hit_record::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), hit_record.normal);

        scattered.set(
            hit_record.p,
            reflected + (Vec3::random_in_unit_sphere() * self.fuzz),
        );
        attenuation.set(self.albedo);

        vec3::dot(scattered.direction(), hit_record.normal) > 0.0
    }
}
