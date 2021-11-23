use std::{cell::RefCell, rc::Rc};

use super::{material::Material, ray::{Ray, Hittable, HitRecord}, vec3::Vec3};

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat_ptr: Rc<RefCell<dyn Material>>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat_ptr: Rc<RefCell<dyn Material>>) -> Self {
        Self{center, radius, mat_ptr}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let direction = ray.direction();

        let a = direction.len_square();
        let half_b = oc.dot_mul(&direction);
        let c = oc.len_square() - self.radius * self.radius;
        
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return false; }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return false; }
        }

        rec.set_t(root);
        rec.set_point(ray.at(rec.t()));
        let outward_normal = (rec.point() - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.set_mat_ptr(Some(Rc::clone(&self.mat_ptr)));
        
        true
    }
}
