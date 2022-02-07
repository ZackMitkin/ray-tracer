use std::rc::Rc;

use crate::{
    materials::material::Material,
    tracer::ray::Ray,
    utils::vec3::{self, Vec3},
};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            front_face: false,
            normal: Vec3::new(),
            p: Vec3::new(),
            t: 0.0,
            material: Option::None,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
