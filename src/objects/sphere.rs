use std::rc::Rc;

use crate::{
    materials::material::Material,
    tracer::ray::Ray,
    utils::vec3::{self, Vec3},
};

use super::{hit_record::HitRecord, hittable::Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, r: f64, material: Option<Rc<dyn Material>>) -> Sphere {
        Sphere {
            center,
            radius: r,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        rec.material = self.material.clone();

        true
    }
}
