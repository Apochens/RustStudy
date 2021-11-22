use crate::render::utils::clamp;
use crate::render::vec3::Vec3;
use std::fs::File;
use std::io::Write;
use std::process::Output;

#[derive(Debug)]
pub struct Color(Vec3);

impl Color {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x * 255.999, y * 255.999, z * 255.999))
    }

    pub fn get_color(&self) -> Vec3 {
        self.0.clone()
    }

    pub fn print_color(&self) {
        println!("{} {} {}", self.0.x() as i32, self.0.y() as i32, self.0.z() as i32);
    }

    pub fn print_color_to_file(&self, file: &mut File) -> std::io::Result<()> {
        match (*file).write(
            format!("{} {} {}\n", 
                        self.0.x() as i32,
                        self.0.y() as i32, 
                        self.0.z() as i32).as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}

impl std::convert::From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color(Vec3::new(
            v.x() * 255.999, 
            v.y() * 255.999, 
            v.z() * 255.999))
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(Vec3::new(
            self.get_color().x() + rhs.get_color().x(),
            self.get_color().y() + rhs.get_color().y(),
            self.get_color().z() + rhs.get_color().z()
        ))
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.get_color().x();
    let g = pixel_color.get_color().y();
    let b = pixel_color.get_color().z();

    let scale = 1. / samples_per_pixel as f32;

    println!("{} {} {}", 
        256. * clamp(r * scale, 0.0, 0.999),
        256. * clamp(g * scale, 0.0, 0.999),
        256. * clamp(b * scale, 0.0, 0.999)
    )
}