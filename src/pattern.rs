use crate::color::Color;
use crate::geometry::{Matrix4x4, Vec4};
use crate::shape::Shape;

pub trait Pattern {
    fn color_at(&self, point: &Vec4) -> Color;
    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color;
}

pub struct StripePattern {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub transform: Matrix4x4,
}

impl StripePattern {
    pub fn new(primary_color: Color, secondary_color: Color, transform: Matrix4x4) -> Self {
        return Self {
            primary_color,
            secondary_color,
            transform,
        };
    }
}

impl Default for StripePattern {
    fn default() -> Self {
        return Self {
            primary_color: Color::new(1.0, 1.0, 1.0),
            secondary_color: Color::new(0.0, 0.0, 0.0),
            transform: Matrix4x4::identity(),
        };
    }
}

impl Pattern for StripePattern {
    fn color_at(&self, point: &Vec4) -> Color {
        if point.x().floor() as i32 % 2 == 0 {
            return self.primary_color;
        }

        return self.secondary_color;
    }

    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color {
        let object_point = shape.transform().invert() * *world_point;
        let pattern_point = self.transform * object_point;

        return self.color_at(&pattern_point);
    }
}

pub struct GradientPattern {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub transform: Matrix4x4,
}

impl GradientPattern {
    pub fn new(primary_color: Color, secondary_color: Color, transform: Matrix4x4) -> Self {
        return Self {
            primary_color,
            secondary_color,
            transform,
        };
    }
}

impl Default for GradientPattern {
    fn default() -> Self {
        return Self {
            primary_color: Color::new(1.0, 1.0, 1.0),
            secondary_color: Color::new(0.0, 0.0, 0.0),
            transform: Matrix4x4::identity(),
        };
    }
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: &Vec4) -> Color {
        let distance = self.secondary_color - self.primary_color;
        let fraction = point.x() - point.x().floor();

        return self.primary_color + distance * fraction;
    }

    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color {
        let object_point = shape.transform().invert() * *world_point;
        let pattern_point = self.transform * object_point;

        return self.color_at(&pattern_point);
    }
}

pub struct RingPattern {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub transform: Matrix4x4,
}

impl RingPattern {
    pub fn new(primary_color: Color, secondary_color: Color, transform: Matrix4x4) -> Self {
        return Self {
            primary_color,
            secondary_color,
            transform,
        };
    }
}

impl Default for RingPattern {
    fn default() -> Self {
        return Self {
            primary_color: Color::new(1.0, 1.0, 1.0),
            secondary_color: Color::new(0.0, 0.0, 0.0),
            transform: Matrix4x4::identity(),
        };
    }
}

impl Pattern for RingPattern {
    fn color_at(&self, point: &Vec4) -> Color {
        if (point.x() * point.x() + point.z() * point.z()).sqrt() as i32 % 2 == 0 {
            return self.primary_color;
        }

        return self.secondary_color;
    }

    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color {
        let object_point = shape.transform().invert() * *world_point;
        let pattern_point = self.transform * object_point;

        return self.color_at(&pattern_point);
    }
}

pub struct CheckeredPattern {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub transform: Matrix4x4,
}

impl CheckeredPattern {
    pub fn new(primary_color: Color, secondary_color: Color, transform: Matrix4x4) -> Self {
        return Self {
            primary_color,
            secondary_color,
            transform,
        };
    }
}

impl Default for CheckeredPattern {
    fn default() -> Self {
        return Self {
            primary_color: Color::new(1.0, 1.0, 1.0),
            secondary_color: Color::new(0.0, 0.0, 0.0),
            transform: Matrix4x4::identity(),
        };
    }
}

impl Pattern for CheckeredPattern {
    fn color_at(&self, point: &Vec4) -> Color {
        if (point.x().floor() + point.y().floor() + point.z().floor()) as i32 % 2 == 0 {
            return self.primary_color;
        }

        return self.secondary_color;
    }

    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color {
        let object_point = shape.transform().invert() * *world_point;
        let pattern_point = self.transform * object_point;

        return self.color_at(&pattern_point);
    }
}

pub struct BlendedPattern {
    pub first_pattern: Box<dyn Pattern>,
    pub second_pattern: Box<dyn Pattern>,
    pub transform: Matrix4x4,
}

impl BlendedPattern {
    pub fn new(first_pattern: Box<dyn Pattern>, second_pattern: Box<dyn Pattern>, transform: Matrix4x4) -> Self {
        return Self {
            first_pattern,
            second_pattern,
            transform,
        };
    }
}

impl Pattern for BlendedPattern {
    fn color_at(&self, point: &Vec4) -> Color {
        let mut result = self.first_pattern.color_at(point) + self.second_pattern.color_at(point);
        result = result * 0.5;
        return result
    }

    fn color_at_object(&self, shape: &dyn Shape, world_point: &Vec4) -> Color {
        let object_point = shape.transform().invert() * *world_point;
        let pattern_point = self.transform * object_point;

        return self.color_at(&pattern_point);
    }
}
