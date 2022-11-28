use std::{rc::Rc, sync::mpsc};

use image::imageops;
use materials::{lambertian::Lambertian, metal::Metal};
use objects::{
    hit_record::HitRecord,
    hittable::{Hittable, HittableList},
    sphere::Sphere,
};
use rayon::prelude::*;
use tracer::{camera::Camera, ray::Ray};
use utils::{
    random::random_f64,
    vec3::{self, Vec3},
};
pub(crate) mod materials;
pub(crate) mod objects;
pub(crate) mod tracer;
pub(crate) mod utils;

extern crate image;

const INFINITY: f64 = f64::MAX;

fn ray_color(ray: &mut Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    let mut rec = HitRecord::new();

    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::from(Vec3::new(), Vec3::new());
        let mut attenuation = Vec3::new();

        let material = rec.material.as_ref().unwrap().to_owned();

        if material.scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&mut scattered, world, depth - 1);
        } else {
            return Vec3::new();
        }
    }
    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: f64 = 100.0;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3::from(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::from(0.7, 0.3, 0.3));
    let material_left = Metal::new(Vec3::from(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Vec3::from(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Vec3::from(0.0, -100.5, -1.0),
        100.0,
        Option::Some(Rc::new(material_ground)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::from(0.0, 0.0, -1.0),
        0.5,
        Option::Some(Rc::new(material_center)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::from(-1.0, 0.0, -1.0),
        0.5,
        Option::Some(Rc::new(material_left)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::from(1.0, 0.0, -1.0),
        0.5,
        Option::Some(Rc::new(material_right)),
    )));

    // camera
    let camera = Camera::new();

    // render
    let mut image = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}/{} ", &j, image_height);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new();

            for _ in 0..samples_per_pixel as u32 {
                let u = ((i as f64) + random_f64()) / (image_width as f64);
                let v = ((j as f64) + random_f64()) / (image_height as f64);

                let mut ray = camera.get_ray(u, v);
                pixel_color += ray_color(&mut ray, &world, max_depth);
            }

            image.put_pixel(
                i,
                j,
                image::Rgb(pixel_color.as_color_sampled(samples_per_pixel)),
            );
        }
    }
    image = imageops::flip_vertical(&image);
    image.save("output.png").unwrap();
}
