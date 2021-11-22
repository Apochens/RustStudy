use rand::{self, Rng};

// Constants
pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

// Utility Functions
pub fn degrees_to_radius(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random() -> f32 { rand::thread_rng().gen() }

pub fn random_in_range(min: f32, max: f32) -> f32 { min + (max - min) * random() }

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {min} else if x > max {max} else {x}
}
