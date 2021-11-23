use super::ray::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>
}

impl HittableList {
    pub fn new() -> Self { Self{objects: vec![]} }
    
    pub fn add(&mut self, object:Rc<RefCell<dyn Hittable>>) {
        self.objects.push(Rc::clone(&object));
    }

    pub fn clear(&mut self) { 
        self.objects.clear(); 
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.as_ref().borrow().hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}