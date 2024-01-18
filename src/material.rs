use crate::{ray::Ray, vector::Vec3D, body::HitRecord};

pub mod materials;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool;

    fn emit(&self) -> Vec3D {
        Vec3D::zero()
    }
}