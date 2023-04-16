use rand::rngs::ThreadRng;

use crate::vec3::{Vec3, Loc3, Colour};
use crate::obj::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub loc: Loc3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Loc3 {
        self.loc + self.dir * t
    }

    pub fn trace(&self, rng: &mut ThreadRng, objects: &Vec<Box<dyn Hittable>>, depth: u16) -> Colour {
        if depth == 0 {
            return Vec3::ZERO;
        }

        for obj in objects {
            if let Some(hit) = obj.hit_test(*self, 0.0..f64::INFINITY) {
                let new_ray = Ray { loc: hit.loc, dir: Vec3::random_on_unit_hemisphere(rng, hit.normal)};
                return new_ray.trace(rng, objects, depth - 1) * 0.5;
            }
        }


        let unit_y = self.dir.normalise().1;
        let t = 0.5 * unit_y + 0.5;

        Vec3::ONES * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
    }
}