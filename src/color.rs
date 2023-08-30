use std::ops::{Add, Mul, Sub};
use crate::util;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        return Self {
            r, 
            g, 
            b,
        };
    }

    pub fn rgb(&self) -> u32 {
        let r = util::clamp_f32(self.r, 0.0, 1.0);
        let g = util::clamp_f32(self.g, 0.0, 1.0);
        let b = util::clamp_f32(self.b, 0.0, 1.0);

        let (r, g, b) = ((255.999 * r) as u32, (255.999 * g) as u32, (255.999 * b) as u32);
        return (r << 16) | (g << 8) | b;
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r + rhs.r;
        let g = self.g+ rhs.g;
        let b = self.b + rhs.b;

        return Self {
            r,
            g,
            b,
        };
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.r - rhs.r;
        let g = self.g - rhs.g;
        let b = self.b - rhs.b;

        return Self {
            r, 
            g,
            b,
        };
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scaler: f32) -> Self::Output {
        let r = self.r * scaler;
        let g = self.g * scaler;
        let b = self.b * scaler;

        return Self {
            r, 
            g, 
            b,
        };
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.r * rhs.r;
        let g = self.g * rhs.g;
        let b = self.b * rhs.b;

        return Self {
            r,
            g,
            b,
        };
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return 
            util::equals_f32(&self.r, &other.r) &&
            util::equals_f32(&self.g, &other.g) &&
            util::equals_f32(&self.b, &other.b);
    }

    fn ne(&self, other: &Self) -> bool {
        return 
            !util::equals_f32(&self.r, &other.r) ||
            !util::equals_f32(&self.g, &other.g) ||
            !util::equals_f32(&self.b, &other.b);
    }
}