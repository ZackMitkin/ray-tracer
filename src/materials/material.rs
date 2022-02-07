use std::fmt::Debug;

use crate::{objects::hit_record::HitRecord, tracer::ray::Ray, utils::vec3::Vec3};

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &mut Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
