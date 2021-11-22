use super::ray::*;
use std::rc::Rc;
use std::cell::RefCell;

struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}