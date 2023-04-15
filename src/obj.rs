use std::ops::Range;

use crate::vec3::{Vec3, Loc3};
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    pub loc: Loc3,
    pub distance: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn correct_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit_test(&self, ray: Ray, acceptable: Range<f64>) -> Option<Hit>;
}

pub struct Sphere {
    pub loc: Loc3,
    pub radius: f64
}

impl Sphere {
    pub fn new(loc: Loc3, radius: f64) -> Self {
        Sphere { loc, radius }
    }
}

impl Hittable for Sphere {
    fn hit_test(&self, ray: Ray, acceptable: Range<f64>) -> Option<Hit> {
        let offset_centre = ray.loc - self.loc;
        let a = ray.dir.magnitude_squared();
        let half_b = Vec3::dot(ray.dir, offset_centre);
        let c = offset_centre.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a * c;
        
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            // further root
            let mut root = (-half_b - sqrt_discriminant) / a;

            if acceptable.contains(&root) {
                let maybe_closer = (-half_b + sqrt_discriminant) / a;
                if acceptable.contains(&maybe_closer) && maybe_closer < root {
                    root = -half_b + sqrt_discriminant / a
                }
                
                let point = ray.at(root);
                let normal = (point - self.loc) * (1.0 / self.radius);

                let mut hit = Hit { loc: point, distance: root, normal, front_face: false };
                hit.correct_face_normal(ray, normal);

                Some(hit)
            } else {
                return None;
            }
        } else {
            None
        }
    }
}