use super::vec3::Vec3;
use super::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical= Vec3::new(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);
        
        Self{origin, horizontal, vertical, lower_left_corner}
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}