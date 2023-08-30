use uuid::Uuid;
use crate::color::Color;
use crate::geometry::Vec4;

pub struct Light {
    pub id: Uuid,
    pub intensity: Color,
    pub position: Vec4,
}

impl Light {
    pub fn point_light(position: Vec4, intensity: Color) -> Self {
        return Self {
            id: Uuid::new_v4(),
            position,
            intensity,
        };
    }
}
