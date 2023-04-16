use crate::vec3::Colour;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Colour>,

    pub samples_per_pixel_correction: u16,
}

impl Image {
    pub fn blank(w: usize, h: usize, samples_per_pixel_correction: u16) -> Self {
        Image {
            w, h,
            data: vec![Colour::ZERO; w * h],
            samples_per_pixel_correction
        }
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut content = format!("P3\n{} {}\n255\n", self.w, self.h);
        
        for i in self.data.iter() {
            let i = *i * (1.0 / self.samples_per_pixel_correction as f64);

            let r = (i.0.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (i.1.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (i.2.clamp(0.0, 1.0) * 255.0) as u8;

            content.push_str(&format!("{} {} {}\n", r, g, b));
        }
        
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }
}