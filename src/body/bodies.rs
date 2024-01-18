use std::{rc::Rc, f32};

use crate::{vector::Vec3D, interval::Interval, material::Material};

use super::{Body, HitRecord};

pub struct Sphere {
    pub center: Vec3D,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Body for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag2();
        let half_b = ray.direction.dot(&oc);
        let c = oc.mag2() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false
        } 
        
        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_discriminant) / a;
        if t.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;

            if t.surrounds(root) {
                return false
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = self.material.clone();
        
        true
    }
}

pub struct Plane {
    pub center: Vec3D,
    pub normal: Vec3D,
    pub material: Rc<dyn Material>,
}

impl Body for Plane {
    fn hit(&self, ray: &crate::ray::Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < f32::EPSILON {
            return false
        }

        let a = (self.center - ray.origin).dot(&self.normal) / denom;

        if t.surrounds(a) {
            return false
        }

        hit_record.t = a;
        hit_record.point = ray.at(a);
        hit_record.set_face_normal(ray, &self.normal);
        hit_record.material = self.material.clone();

        true
    }
}