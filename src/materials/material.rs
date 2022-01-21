use crate::{objects::hittable::HitRecord, tracer::ray::Ray, utils::vec3::Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray);
}
