use crate::geometry::{Matrix4x4, Vec4};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: Vec4, direction: Vec4) -> Self {
        return Self {
            origin,
            direction,
        };
    }

    pub fn at(&self, t: f32) -> Vec4 {
        return self.origin + self.direction * t;
    }

    pub fn reflect(&self, normalv: &Vec4) -> Vec4 {
        return self.direction.reflect(normalv);
    }

    pub fn transform(&self, matrix: Matrix4x4) -> Self {
        return Self {
            origin: matrix * self.origin,
            direction: matrix * self.direction,
        };
    }
}