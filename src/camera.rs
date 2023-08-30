use crate::canvas::Canvas;
use crate::geometry::{Matrix4x4, Vec4};
use crate::ray::Ray;
use crate::world::World;

pub struct Camera {
    pub hsize: f32,
    pub vsize: f32,
    pub field_of_view: f32,
    pub transform: Matrix4x4,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    pub fn new(hsize: f32, vsize: f32, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;

        let half_width: f32;
        let half_height: f32;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize;

        return Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4x4::identity(),
            pixel_size,
            half_width,
            half_height,
        };
    }

    pub fn set_view_transform(&mut self, from: Vec4, to: Vec4, up: Vec4) {
        let forward = (to - from).normalize();
        let upn= up.normalize();
        let left = forward.cross(&upn);
        let true_up = left.cross(&forward);

        let orientation = Matrix4x4::new([
            *left.x(), *left.y(), *left.z(), 0.0,
            *true_up.x(), *true_up.y(), *true_up.z(), 0.0,
            -*forward.x(), -*forward.y(), -*forward.z(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        self.transform = orientation * Matrix4x4::translation(-from.x(), -from.y(), -from.z());
    }

    pub fn ray_for_pixel(&self, px: f32, py: f32) -> Ray {
        let xoffset = (px + 0.5) * self.pixel_size;
        let yoffset = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.invert() * Vec4::point(world_x, world_y, -1.0);
        let origin = self.transform.invert() * Vec4::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        return Ray::new(origin, direction);
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
         for y in 0..self.vsize as usize - 1 {
            for x in 0..self.hsize as usize - 1 {
                let ray = self.ray_for_pixel(x as f32, y as f32);
                let color = world.color_at(ray, 5);
                image.set_color(x, y, &color);
            }
         }

         return image;
    }
}