#![allow(dead_code)]

mod vec3;
mod ray;
mod image;
mod camera;
mod obj;

use camera::Camera;
use obj::{Sphere, Hittable};
use vec3::Vec3;

fn main() {
    let camera = Camera::sensible();

    let scene: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::B, 0.5)),
        Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
    ];

    let image  = camera.render(&scene);
    image.write_ppm("image.ppm").unwrap();
}
