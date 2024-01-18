pub mod body_list;
pub mod bodies;

use std::rc::Rc;

use crate::{vector::Vec3D, ray::Ray, interval::Interval, material::{Material, materials::Base}};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3D,
    pub normal: Vec3D,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3D::zero(),
            normal: Vec3D::zero(),
            material: Rc::new(Base {}),
            t: 0.0,
            front_face: false,
        }
    }

    /// 'outward_normal' must be normalized.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3D) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Body {
    fn hit(&self, ray: &Ray, t: Interval, hit_record: &mut HitRecord) -> bool;
}