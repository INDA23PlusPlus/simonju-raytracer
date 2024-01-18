use std::f32::INFINITY;

use macroquad::color::Color;

use crate::{vector::Vec3D, body::{HitRecord, Body}, interval::Interval};

const MAX_DEPTH: usize = 50;

const BACKGROUND_COLOR: Vec3D = Vec3D::new(0.1, 0.2, 0.7);

pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: &Vec3D, direction: &Vec3D) -> Self {
        Self { origin: *origin, direction: *direction }
    }

    #[inline]
    pub fn at(&self, t: f32) -> Vec3D {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &dyn Body, depth: usize) -> Vec3D {
        let mut hit_record = HitRecord::new();

        if depth > MAX_DEPTH {
            return Vec3D::zero()
        }

        if !world.hit(self, Interval::new(0.001, f32::INFINITY), &mut hit_record) {
            return BACKGROUND_COLOR
        }

        let mut scattered = Ray::new(&Vec3D::zero(), &Vec3D::zero());
        let mut attenuation = Vec3D::zero();
        let emitted_color = hit_record.material.emit();

        if !hit_record.material.scatter(&self, &mut scattered, &mut attenuation, &hit_record) {
            return emitted_color
        }

        let scattered_color = attenuation * scattered.color(world, depth + 1);

        emitted_color + scattered_color

        // let a = 0.5 * (self.direction.unit().y + 1.0);
        // (1.0 - a) * Vec3D::one() + a * Vec3D::new(0.5, 0.7, 1.0)
    }
}