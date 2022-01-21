use image::imageops;
use objects::{
    hittable::{HitRecord, Hittable, HittableList},
    sphere::Sphere,
};
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

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::from(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 150;

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0)));

    // camera
    let camera = Camera::new();

    // render
    let mut image = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}/{} ", &j, image_height);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new();

            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + random_f64()) / (image_width as f64);
                let v = ((j as f64) + random_f64()) / (image_height as f64);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }

            let pixel = image.get_pixel_mut(i, j);
            *pixel = image::Rgb(pixel_color.as_color_sampled(samples_per_pixel as f64));
        }
    }
    image = imageops::flip_vertical(&image);
    image.save("output.png").unwrap();
}
