use std::ops::Neg;

use super::utils::{random, random_in_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {

    pub fn new_empty() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }

    pub fn len_square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_square().sqrt()
    }

    pub fn dot_mul(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross_mul(&self, rhs: Vec3) -> Vec3 {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    // Unitization
    pub fn to_unit(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(random_in_range(min, max), random_in_range(min, max), random_in_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut ret = Vec3::new_empty();
        loop {
            let p = Vec3::random_in_range(-1., 1.);
            if p.len_square() >= 1. { continue; }
            ret = p;
            break;
        }
        ret
    }

    pub fn random_unit_vec3() -> Vec3 {
        let mut vec3 = Self::random_in_unit_sphere();
        vec3.to_unit();
        vec3
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot_mul(normal) > 0. { in_unit_sphere }
        else { -in_unit_sphere }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8f32;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2. * self.dot_mul(normal) * (*normal)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(
            self.x + rhs.x(),
            self.y + rhs.y(),
            self.z + rhs.z()
        )
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z()
        )
    }
}

// Vec3 * f32
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}

// f32 * Vec3
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z()
        )
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}