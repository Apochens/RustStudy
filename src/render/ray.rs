use crate::render::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self{origin, direction}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin.clone() + t * self.direction.clone()
    }
}

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot_mul(outward_normal) < 0.0;
        self.normal = if self.front_face == true { *outward_normal } else { -(*outward_normal) }
    }
}

impl HitRecord {
    pub fn point(&self) -> Vec3 { self.point }
    pub fn normal(&self) -> Vec3 { self.normal }
    pub fn t(&self) -> f32 { self.t }
    pub fn set_point(&mut self, point: Vec3) { self.point = point; }
    pub fn set_normal(&mut self, normal: Vec3) { self.normal = normal; }
    pub fn set_t(&mut self, t: f32) { self.t = t; }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}