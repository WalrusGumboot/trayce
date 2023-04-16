use crate::image::Image;
use crate::obj::Hittable;
use crate::ray::Ray;
use crate::vec3::{Colour, Loc3, Vec3};

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;

pub struct Camera {
    pub loc: Loc3,

    pub img_width: usize,
    pub img_height: usize,

    pub viewport_width: f64,
    pub viewport_height: f64,

    pub focal_length: f64,

    pub samples_per_pixel: u16,

    pub max_bounces: u16
}

impl Camera {
    pub fn sensible() -> Self {
        Camera {
            loc: Loc3::ZERO,
            img_width: 640,
            img_height: 480,
            viewport_width: 2.0,
            viewport_height: 1.5,
            focal_length: 1.0,
            samples_per_pixel: 100,
            max_bounces: 5,
        }
    }

    pub fn horizontal(&self) -> Vec3 {
        self.loc + Vec3::R * self.viewport_width
    }

    pub fn vertical(&self) -> Vec3 {
        self.loc + Vec3::U * self.viewport_height
    }

    pub fn lower_left_corner(&self) -> Loc3 {
        self.loc - self.horizontal() * 0.5 - self.vertical() * 0.5 - Vec3::F * self.focal_length
    }

    pub fn render(&self, objects: &Vec<Box<dyn Hittable>>) -> Image {
        let mut image_data: Vec<Colour> = Vec::new();

        let hor = self.horizontal();
        let ver = self.vertical();
        let llc = self.lower_left_corner();

        let mut rng = rand::thread_rng();

        let num_pixels = self.img_width * self.img_height;

        let bar = ProgressBar::new(num_pixels as u64 * self.samples_per_pixel as u64)
            .with_prefix("Rendering:")
            .with_style(
                ProgressStyle::with_template(
                    "Rendering: [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos} / {len})",
                )
                .unwrap()
            );

        for idx in 0..num_pixels {
            let mut colour: Colour = Vec3::ZERO;

            for _ in 0..self.samples_per_pixel {
                let x = idx % self.img_width;
                let y = self.img_height - idx / self.img_width;

                let u_off: f64 = rng.gen();
                let v_off: f64 = rng.gen();

                let u = (x as f64 + u_off) / (self.img_width as f64 - 1.0);
                let v = (y as f64 + v_off) / (self.img_height as f64 - 1.0);

                let ray = Ray {
                    loc: self.loc,
                    dir: llc + hor * u + ver * v - self.loc,
                };

                colour += ray.trace(&mut rng, objects, self.max_bounces);
            }

            image_data.push(colour);

            bar.inc(self.samples_per_pixel as u64);
        }

        bar.finish_with_message("Generated image.");

        Image {
            w: self.img_width,
            h: self.img_height,
            data: image_data,
            samples_per_pixel_correction: self.samples_per_pixel,
        }
    }
}
