use std::ops::Neg;

use crate::random;

use super::*;

pub struct Base {}

impl Material for Base {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool {
        true
    }
}

pub struct Lambertian { pub albedo: Vec3D }

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3D::random_unit();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal
        }

        *ray_out = Ray::new(&hit_record.point, &scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal { pub albedo: Vec3D, pub fuzz: f32 }

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool {
        let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
        *ray_out = Ray::new(&hit_record.point, &(reflected + self.fuzz * Vec3D::random_unit()));
        *attenuation = self.albedo;
        
        ray_out.direction.dot(&hit_record.normal) > 0.0
    }
}

pub struct Dielectric { pub refraction_index: f32 }

impl Dielectric {
    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool {
        *attenuation = Vec3D::one();
        let refraction_ratio = if hit_record.front_face { 1.0 / self.refraction_index } 
            else { self.refraction_index };

        let unit_direction = ray_in.direction.unit();

        let cos_theta = unit_direction.dot(&hit_record.normal).neg().min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 
        || Self::reflectance(cos_theta, refraction_ratio) > random() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        *ray_out = Ray::new(&hit_record.point, &direction);

        true
    }
}

pub struct Light { pub color: Vec3D }

impl Material for Light {
    fn scatter(&self, ray_in: &Ray, ray_out: &mut Ray, attenuation: &mut Vec3D, hit_record: &HitRecord) -> bool {
        false
    }

    fn emit(&self) -> Vec3D {
        self.color
    }
}