use std::ops::{Div, Neg};

use macroquad::{texture::{Image, Texture2D, draw_texture_ex, DrawTextureParams}, color::{Color, BLANK, WHITE}, window::clear_background, text::draw_text, time::get_fps, shapes::{draw_rectangle_ex, DrawRectangleParams}, math::Vec2};
use crate::{body::Body, vector::Vec3D, ray::Ray, random, degrees_to_radians};

pub struct Camera {
    aspect_ratio: f32,
    image_width: usize,
    image_height: usize,

    center: Vec3D,
    direction: Vec3D,
    image: Image,
    texture: Texture2D,

    viewport_origin: Vec3D,
    viewport_width: f32,
    viewport_height: f32,
    viewport_u: Vec3D,
    viewport_v: Vec3D,

    pixel_origin: Vec3D,
    pixel_delta_u: Vec3D,
    pixel_delta_v: Vec3D,

    samples_per_pixel: usize,

    draw_parameters: DrawTextureParams,

    vertical_field_of_view: f32,
    look_from: Vec3D,
    look_to: Vec3D,
    view_up: Vec3D,

    u: Vec3D,
    v: Vec3D,
    w: Vec3D,

    defocus_angle: f32,
    focus_distance: f32,

    defocus_disk_u: Vec3D,
    defocus_disk_v: Vec3D,

    image_scaling: f32,

    frame_count: usize,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0; // ideal aspect ratio
        let image_width: usize = 720;
        let image_height: usize = {
            let x = (image_width as f32 / aspect_ratio) as usize;
            if x < 1 { 1 } else { x }
        };

        let look_from = Vec3D::new(0.0, 0.0, 2.0);
        let look_to = Vec3D::zero();
        let view_up = Vec3D::y_unit();

        let view_direction = look_from - look_to;

        let defocus_angle = 0.0;
        let focus_distance = 3.0;

        let vertical_field_of_view: f32 = 90.0;
        let theta: f32 = degrees_to_radians(vertical_field_of_view);
        let h = theta.div(2.0).tan();
        let viewport_height: f32 = 2.0 * h * focus_distance;
        let viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;

        let w = view_direction.unit();
        let u = view_up.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u: Vec3D = viewport_width * u;
        let viewport_v: Vec3D = viewport_height.neg() * v;

        let image: Image = {
            let mut image = Image::empty();

            image.width = image_width as u16;
            image.height = image_height as u16;
            image.bytes = vec![0; image_width * image_height * 4];

            image
        };

        let texture: Texture2D = Texture2D::from_image(&image);

        let center: Vec3D = look_from;
        let direction: Vec3D = view_direction.unit();

        let pixel_delta_u: Vec3D = viewport_u / image_width as f32;
        let pixel_delta_v: Vec3D = viewport_v / image_height as f32;
    
        let viewport_origin: Vec3D = center 
            - (w * focus_distance )
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);

        let pixel_origin: Vec3D = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel: usize = 100;

        let image_scaling = 1.0;

        let draw_parameters = {
            let mut draw_parameters = DrawTextureParams::default();

            draw_parameters.dest_size = Some(Vec2::new(
                image_width as f32 * image_scaling, 
                image_height as f32 * image_scaling
            ));

            draw_parameters
        };

        let defocus_radius = focus_distance * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let frame_count = 0;

        Self {
            aspect_ratio,
            image_width,
            image_height,

            center,
            direction,
            image,
            texture,

            viewport_origin,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,

            pixel_origin,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,

            draw_parameters,

            vertical_field_of_view,

            look_from,
            look_to,
            view_up,

            u,
            v,
            w,

            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,

            image_scaling,

            frame_count,
        }
    }

    pub fn get_image_width(&self) -> usize {
        self.image_width
    }

    pub fn get_scaled_image_width(&self) -> usize {
        (self.image_width as f32 * self.image_scaling) as usize
    }

    pub fn get_image_height(&self) -> usize {
        self.image_height
    }

    pub fn get_scaled_image_height(&self) -> usize {
        (self.image_height as f32 * self.image_scaling) as usize
    }

    pub fn render(&mut self, world: &dyn Body) {
        clear_background(BLANK);

        self.frame_count += 1;
        
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut pixel_color = Vec3D::zero();

                for _ in 0..self.samples_per_pixel {
                    pixel_color += self.get_ray(x, y).color(world, 0);
                }

                let pixel_color = pixel_color / self.samples_per_pixel as f32;

                let pixel = self.image.get_pixel(x as u32, y as u32);

                let pixel_color = (pixel_color / self.frame_count as f32) 
                    + (Vec3D::new(pixel.r, pixel.g, pixel.b) * ((self.frame_count - 1) as f32 / self.frame_count as f32));

                let color = Color::new(pixel_color.x, pixel_color.y, pixel_color.z, 1.0);

                self.image.set_pixel(x as u32, y as u32, color);
            }
        }

        self.texture.update(&self.image);

        draw_texture_ex(&self.texture, 0.0, 0.0, WHITE, self.draw_parameters.clone());

        draw_text(&format!("FPS: {}", get_fps()), 5.0, 20.0, 30.0, WHITE);
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_center = self.pixel_origin
            + (x as f32 * self.pixel_delta_u)
            + (y as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - self.center;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3D {
        let px = -0.5 + random();
        let py = -0.5 + random();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Vec3D {
        let p = Vec3D::random_in_unit_disc();

        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}