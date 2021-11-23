use super::ray::{Ray, HitRecord};
use super::color::Color;
use super::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self{albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vec3();
        if scatter_direction.near_zero() { scatter_direction = rec.normal(); }
        *scattered = Ray::new(rec.point(), scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self{albedo, fuzz: if fuzz < 1.0 {fuzz} else {1.0}}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut ray_in_dir = ray_in.direction();
        ray_in_dir.to_unit();
        let reflected = ray_in_dir.reflect(&rec.normal());
        *scattered = Ray::new(rec.point(), reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot_mul(&rec.normal()) > 0.
    }
}
