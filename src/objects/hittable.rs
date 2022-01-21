use std::fmt::Debug;

use crate::{
    materials::material::Material,
    tracer::ray::Ray,
    utils::vec3::{self, Vec3},
};
#[derive(Debug, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    //pub material: Vec<Box<dyn Material>>,
}

impl Clone for HitRecord {
    fn clone(&self) -> HitRecord {
        *self
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            front_face: false,
            normal: Vec3::new(),
            p: Vec3::new(),
            t: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(&r.direction(), &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, rec) {
                hit = true;
                closest_so_far = rec.t;
            }
        }

        return hit;
    }
}
