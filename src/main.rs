use crate::tracer::vec3::{dot, unit_vector};
use image::imageops;
use objects::{
    hittable::{HitRecord, Hittable, HittableList},
    sphere::Sphere,
};
use tracer::{
    ray::Ray,
    vec3::{self, Vec3},
};
pub(crate) mod objects;
pub(crate) mod tracer;

extern crate image;

const INFINITY: f64 = f64::MAX;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::from(1.0, 1.0, 1.0));
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

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0)));

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new();
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    // render
    let mut image = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64);
            let v = (j as f64) / (image_height as f64);

            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::from(origin, direction);

            let pixel = image.get_pixel_mut(i, j);
            *pixel = image::Rgb(ray_color(&r, &world).as_color());
        }
    }
    image = imageops::flip_vertical(&image);
    image.save("output.png").unwrap();
}
