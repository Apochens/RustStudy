use super::{vec3::Vec3, ray::{Ray, Hittable, HitRecord}};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self{center, radius}
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
        
        true
    }
}
