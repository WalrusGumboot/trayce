use crate::ray::Ray;
use crate::vec3::{Vec3, Loc3, Colour};
use crate::image::Image;
use crate::obj::Hittable;

pub struct Camera {
    pub loc: Loc3,
    
    pub img_width: usize,
    pub img_height: usize,

    pub viewport_width: f64,
    pub viewport_height: f64,
    
    pub focal_length: f64,

}

impl Camera {
    pub fn sensible() -> Self {
        Camera { 
            loc: Loc3::ZERO,
            img_width: 640, 
            img_height: 480,
            viewport_width: 2.0,
            viewport_height: 1.5,
            focal_length: 1.0 
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
        
        for idx in 0..(self.img_width * self.img_height) {
            let x = idx % self.img_width;
            let y = self.img_height - idx / self.img_width;
            
            let u = x as f64 / (self.img_width as f64 - 1.0);
            let v = y as f64 / (self.img_height as f64 - 1.0);

            let ray = Ray {
                loc: self.loc,
                dir: llc + hor * u + ver * v - self.loc
            };
            
            image_data.push(ray.trace(objects));
        }

        Image { w: self.img_width, h: self.img_height, data: image_data }
    }
}