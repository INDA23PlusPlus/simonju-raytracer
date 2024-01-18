mod vector;
mod ray;
mod body;
mod interval;
mod camera;
mod material;

use std::{rc::Rc, cell::RefCell, ops::Neg};
use body::{body_list::BodyList, bodies::{Sphere, Plane}, Body};
use camera::Camera;
use macroquad::{prelude::*, miniquad::window::set_window_size, rand::{RandomRange}};
use material::materials::{Base, Lambertian, Metal, Dielectric, Light};
use vector::Vec3D;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random() -> f32 {
    f32::gen_range(0.0, 1.0)
}

pub fn random_neg_pos() -> f32 {
    f32::gen_range(-1.0, 1.0)
}

#[macroquad::main("raytracing")]
async fn main() {
    let mut camera = Camera::new();

    set_window_size(
        camera.get_scaled_image_width() as u32, 
        camera.get_scaled_image_height() as u32
    );

    let mut world = BodyList::new();

    let plane = Rc::new(RefCell::new(Plane {
        center: Vec3D::new(0.0, -1.0, 0.0),
        normal: Vec3D::y_unit(),
        material: Rc::new(Lambertian { albedo: Vec3D::new(0.2, 1.0, 0.1) })
    }));

    let sphere = Rc::new(RefCell::new(Sphere {
        center: Vec3D::zero(),
        radius: 1.0,
        material: Rc::new(Lambertian { albedo: Vec3D::new(0.8, 0.2, 0.2) })
    }));

    let light = Rc::new(RefCell::new(Sphere {
        center: Vec3D::new(-40.0, 40.0, 40.0), 
        radius: 10.0,
        material: Rc::new(Light { color: Vec3D::one() * 30.0 }),
    }));

    world.push(plane.clone());
    world.push(sphere.clone());
    world.push(light.clone());

    loop {
        camera.render(&world);

        next_frame().await
    }
}