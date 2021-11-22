use crate::render::vec3::Vec3;
use std::fs::File;
use std::io::Write;

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