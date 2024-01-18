use std::ops::Neg;

use macroquad::math::Vec3;

use crate::{random, random_neg_pos};

mod operators;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    #[inline(always)]
    pub const fn x_unit() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }

    #[inline(always)]
    pub const fn y_unit() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }

    #[inline(always)]
    pub const fn z_unit() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }

    #[inline]
    pub fn random() -> Self {
        Self { x: random_neg_pos(), y: random_neg_pos(), z: random_neg_pos() }
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random();
            if p.mag2() < 1.0 { return p }
        }
    }

    #[inline]
    pub fn random_in_unit_disc() -> Self {
        loop {
            let p = Vec3D::new(random_neg_pos(), random_neg_pos(), 0.0);
            if p.mag2() < 1.0 { return p }
        }
    }

    #[inline]
    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3D) -> Vec3D {
        let on_unit_sphere = Self::random_unit();

        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub fn is_near_zero(&self) -> bool {
        (self.x.abs() <= f32::EPSILON) && (self.y.abs() <= f32::EPSILON) && (self.z.abs() <= f32::EPSILON)
    }

    #[inline]
    pub fn mag(&self) -> f32 {
        self.mag2().sqrt()
    }

    #[inline]
    pub fn mag2(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn unit(&self) -> Self {
        *self / self.mag()
    }

    #[inline]
    pub fn dot(&self, vector: &Self) -> f32 {
        (self.x * vector.x) + (self.y * vector.y) + (self.z * vector.z)
    }

    #[inline]
    pub fn cross(&self, vector: &Self) -> Self {
        Self {
            x: self.y * vector.z - self.z * vector.y,
            y: self.z * vector.x - self.x * vector.z,
            z: self.x * vector.y - self.y * vector.x,
        }
    }

    #[inline]
    pub fn proj(&self, vector: &Self) -> Self {
        *vector * (self.dot(vector) / vector.mag2())
    }

    #[inline]
    pub fn x_proj(&self) -> Self {
        let x_unit = Self::x_unit();
        x_unit * self.dot(&x_unit)
    }

    #[inline]
    pub fn y_proj(&self) -> Self {
        let y_unit = Self::y_unit();
        y_unit * self.dot(&y_unit)
    }

    #[inline]
    pub fn z_proj(&self) -> Self {
        let z_unit = Self::z_unit();
        z_unit * self.dot(&z_unit)
    }

    // normal must be a unit vector
    #[inline]
    pub fn reflect(&self, normal: &Vec3D) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    #[inline]
    pub fn refract(&self, normal: &Vec3D, etai_over_etat: f32) -> Vec3D {
        let cos_theta = self.dot(normal).neg().min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = (1.0 - r_out_perp.mag2()).sqrt().neg() * *normal;

        r_out_perp + r_out_parallel
    }

}