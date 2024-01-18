use std::{rc::Rc, cell::RefCell};

use crate::interval::Interval;

use super::{Body, HitRecord};

pub struct BodyList {
    bodies: Vec<Rc<RefCell<dyn Body>>>
}

impl BodyList {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn push(&mut self, body: Rc<RefCell<dyn Body>>) {
        self.bodies.push(body)
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<dyn Body>>> {
        self.bodies.pop()
    }

    pub fn clear(&mut self) {
        self.bodies.clear()
    }
}

impl Body for BodyList {
    fn hit(&self, ray: &crate::ray::Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::new();
        let mut hit = false;
        let mut closest = t.max;

        for body in &self.bodies {
            if body.borrow().hit(ray, Interval::new(t.min, closest), &mut temp_hit_record) {
                hit = true;
                closest = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        hit
    }
}