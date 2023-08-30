use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::util;

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    elements: [f32; 4],
}

impl Vec4 {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        return Self {
            elements: [x, y, z, 1.0],
        };
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        return Self {
            elements: [x, y, z, 0.0],
        };
    }

    pub fn raw(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self {
            elements: [x, y, z, w],
        };
    }

    pub fn x(&self) -> &f32 {
        return &self.elements[0];
    }

    pub fn y(&self) -> &f32 {
        return &self.elements[1];
    }

    pub fn z(&self) -> &f32 {
        return &self.elements[2];
    }

    pub fn w(&self) -> &f32 {
        return &self.elements[3];
    }    

    pub fn magnitude(&self) -> f32 {
        return (self.x()*self.x() + self.y()*self.y() + self.z()*self.z() + self.w()*self.w()).sqrt();
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        return Self {
            elements: [
                self.elements[0] / magnitude, 
                self.elements[1] / magnitude, 
                self.elements[2] / magnitude, 
                self.elements[3] / magnitude,
            ]
        };
    }

    pub fn dot(&self, other: &Vec4) -> f32 {
        return 
            &self.elements[0] * &other.elements[0] +
            &self.elements[1] * &other.elements[1] +
            &self.elements[2] * &other.elements[2] +
            &self.elements[3] * &other.elements[3];      
    } 

    pub fn cross(&self, other: &Vec4) -> Self {
        return Vec4::vector(
            &self.elements[1] * &other.elements[2] - &self.elements[2] * &other.elements[1], 
            &self.elements[2] * &other.elements[0] - &self.elements[0] * &other.elements[2], 
            &self.elements[0] * &other.elements[1] - &self.elements[1] * &other.elements[0],
        );
    }

    pub fn reflect(&self, normal: &Vec4) -> Vec4 {
        return *self - *normal * 2.0 * self.dot(&normal);
    }

    pub fn refract(&self, normalv: &Vec4, n_enter: f32, n_exit: f32) -> Vec4 {
        let n_ratio = n_enter / n_exit;
        let cos_theta = f32::min((-*self).dot(normalv), 1.0);
        let r_out_perp = (*self + *normalv * cos_theta) * n_ratio;
        let r_out_parallel = *normalv * -((1.0 - r_out_perp.magnitude()).abs()).sqrt();
        return r_out_perp + r_out_parallel;
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Vec4::raw(self.x()+rhs.x(), self.y()+rhs.y(), self.z()+rhs.z(), self.w()+rhs.w());
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        return Vec4::raw(self.x() + rhs, self.y() + rhs, self.z() + rhs, *self.w());
    }
}

impl Sub for Vec4{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vec4::raw(self.x()-rhs.x(), self.y()-rhs.y(), self.z()-rhs.z(), self.w()-rhs.w());
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        return Vec4::raw(self.x() - rhs, self.y() - rhs, self.z() - rhs, *self.w());
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Vec4::raw(-*self.x(), -*self.y(), -*self.z(), -*self.w());
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, scaler: f32) -> Self::Output {
        return Vec4::raw(self.x()*scaler, self.y()*scaler, self.z()*scaler, self.w()*scaler);
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, scaler: f32) -> Self::Output {
        return Vec4::raw(self.x()/scaler, self.y()/scaler, self.z()/scaler, self.w()/scaler);
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        return 
            util::equals_f32(&self.x(), &other.x()) &&
            util::equals_f32(&self.y(), &other.y()) &&
            util::equals_f32(&self.z(), &other.z()) &&
            util::equals_f32(&self.w(), &other.w());
    }

    fn ne(&self, other: &Self) -> bool {
        return 
        !util::equals_f32(&self.x(), &other.x()) ||
        !util::equals_f32(&self.y(), &other.y()) ||
        !util::equals_f32(&self.z(), &other.z()) ||
        !util::equals_f32(&self.w(), &other.w()); 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2 {
    mat: [f32; 4],
}

impl Matrix2x2 {
    pub fn new(mat: [f32; 4]) -> Self {
        return Self {
            mat,
        };
    }

    pub fn get(&self, r: usize, c: usize) -> &f32 {
        return &self.mat[c + r * 2];
    }

    pub fn determinant(&self) -> f32 {
        return &self.mat[0] * &self.mat[3] - &self.mat[1] * &self.mat[2];
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        return 
            util::equals_f32(&self.mat[0], &other.mat[0]) &&
            util::equals_f32(&self.mat[1], &other.mat[1]) &&
            util::equals_f32(&self.mat[2], &other.mat[2]) &&
            util::equals_f32(&self.mat[3], &other.mat[3]);
    }

    fn ne(&self, other: &Self) -> bool {
        return 
            !util::equals_f32(&self.mat[0], &other.mat[0]) ||
            !util::equals_f32(&self.mat[1], &other.mat[1]) ||
            !util::equals_f32(&self.mat[2], &other.mat[2]) ||
            !util::equals_f32(&self.mat[3], &other.mat[3]);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    mat: [f32; 9],
}

impl Matrix3x3 {
    pub fn new(mat: [f32; 9]) -> Self {
        return Self {
            mat,
        };
    }

    pub fn get(&self, r: usize, c: usize) -> &f32 {
        return &self.mat[c + r * 3];
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut mat: [f32; 4] = [0.0; 4];
        let mut index: usize = 0;

        for r in 0..3 {
            for c in 0..3 {
                if r != row && c != col {
                    mat[index] = *self.get(r, c);
                    index += 1;
                }
            }
        }

        return Matrix2x2 { mat };
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let sub_matrix = self.submatrix(row, col);
        return sub_matrix.determinant();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let determinant = self.minor(row, col);
        if (row + col) % 2 != 0 {
            return -determinant;
        }
        return determinant;
    }

    pub fn determinant(&self) -> f32 {
        return 
            self.get(0, 0) * self.cofactor(0, 0) +
            self.get(0, 1) * self.cofactor(0, 1) +
            self.get(0, 2) * self.cofactor(0, 2);
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        return 
            util::equals_f32(&self.mat[0], &other.mat[0]) &&
            util::equals_f32(&self.mat[1], &other.mat[1]) &&
            util::equals_f32(&self.mat[2], &other.mat[2]) &&
            util::equals_f32(&self.mat[3], &other.mat[3]) &&
            util::equals_f32(&self.mat[4], &other.mat[4]) &&
            util::equals_f32(&self.mat[5], &other.mat[5]) &&
            util::equals_f32(&self.mat[6], &other.mat[6]) &&
            util::equals_f32(&self.mat[7], &other.mat[7]) &&
            util::equals_f32(&self.mat[8], &other.mat[8]);
    }

    fn ne(&self, other: &Self) -> bool {
        return 
            !util::equals_f32(&self.mat[0], &other.mat[0]) ||
            !util::equals_f32(&self.mat[1], &other.mat[1]) ||
            !util::equals_f32(&self.mat[2], &other.mat[2]) ||
            !util::equals_f32(&self.mat[3], &other.mat[3]) ||
            !util::equals_f32(&self.mat[4], &other.mat[4]) ||
            !util::equals_f32(&self.mat[5], &other.mat[5]) ||
            !util::equals_f32(&self.mat[6], &other.mat[6]) ||
            !util::equals_f32(&self.mat[7], &other.mat[7]) ||
            !util::equals_f32(&self.mat[8], &other.mat[8]);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    mat: [f32; 16],
}

impl Matrix4x4 {
    pub fn new(mat: [f32; 16]) -> Self {
        return Self {
            mat,
        };
    }

    pub fn identity() -> Self {
        return Self {mat: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn get(&self, r: usize, c: usize) -> &f32 {
        return &self.mat[c + r * 4];
    }

    pub fn transpose(&self) -> Self {
        return Self {mat: [
            *self.get(0, 0), *self.get(1, 0), *self.get(2, 0), *self.get(3, 0),
            *self.get(0, 1), *self.get(1, 1), *self.get(2, 1), *self.get(3, 1),
            *self.get(0, 2), *self.get(1, 2), *self.get(2, 2), *self.get(3, 2),
            *self.get(0, 3), *self.get(1, 3), *self.get(2, 3), *self.get(3, 3),
        ]};
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut mat: [f32; 9] = [0.0; 9];
        let mut index: usize = 0;

        for r in 0..4 {
            for c in 0..4 {
                if r != row && c != col {
                    mat[index] = *self.get(r, c);
                    index += 1;
                }
            }
        }

        return Matrix3x3 { mat };
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let sub_matrix = &self.submatrix(row, col);
        return sub_matrix.determinant();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let determinant = self.minor(row, col);
        if (row + col) % 2 != 0 {
            return -determinant;
        }
        return determinant;
    }

    pub fn determinant(&self) -> f32 {
        return 
            self.get(0, 0) * self.cofactor(0, 0) +
            self.get(0, 1) * self.cofactor(0, 1) +
            self.get(0, 2) * self.cofactor(0, 2) +
            self.get(0, 3) * self.cofactor(0, 3);
    }

    pub fn is_invertible(&self) -> bool {
        return !util::equals_f32(&self.determinant(), &0.0);
    }

    pub fn invert(&self) -> Self {
        if !self.is_invertible(){
            panic!();
        }

        let det = self.determinant();

        let cofactor_matrix = Matrix4x4::new([
            self.cofactor(0, 0) / det, self.cofactor(0, 1) / det, self.cofactor(0, 2) / det, self.cofactor(0, 3) / det,
            self.cofactor(1, 0) / det, self.cofactor(1, 1) / det, self.cofactor(1, 2) / det, self.cofactor(1, 3) / det,
            self.cofactor(2, 0) / det, self.cofactor(2, 1) / det, self.cofactor(2, 2) / det, self.cofactor(2, 3) / det,
            self.cofactor(3, 0) / det, self.cofactor(3, 1) / det, self.cofactor(3, 2) / det, self.cofactor(3, 3) / det,
        ]);

        return cofactor_matrix.transpose();
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        return Self {mat: [
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        return Matrix4x4::translation(x, y, z) * *self;
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        return Self {mat: [
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn scaling(&self, x: f32, y: f32, z: f32) -> Self {
        return Matrix4x4::scale(x, y, z) * *self;
    }

    pub fn rotatation_x(radians: f32) -> Self {
        return Self {mat: [
            1.0, 0.0, 0.0, 0.0,
            0.0, radians.cos(), -(radians.sin()), 0.0,
            0.0, radians.sin(), radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn rotate_x(&self, radians: f32) -> Self {
        return Matrix4x4::rotatation_x(radians) * *self;
    }

    pub fn rotatation_y(radians: f32) -> Self {
        return Self {mat: [
            radians.cos(), 0.0, radians.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -(radians.sin()), 0.0, radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn rotate_y(&self, radians: f32) -> Self {
        return Matrix4x4::rotatation_y(radians) * *self;
    }

    pub fn rotatation_z(radians: f32) -> Self {
        return Self {mat: [
            radians.cos(), -(radians.sin()), 0.0, 0.0,
            radians.sin(), radians.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn rotate_z(&self, radians: f32) -> Self {
        return Matrix4x4::rotatation_z(radians) * *self;
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        return Self {mat: [
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]};
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        return Matrix4x4::shearing(xy, xz, yx, yz, zx, zy) * *self;
    }

    pub fn view_transformation(from: Vec4, to: Vec4, up: Vec4) -> Matrix4x4 {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(&upn);
        let true_up = left.cross(&forward);

        let orientation = Matrix4x4::new([
            *left.x(), *left.y(), *left.z(), 0.0,
            *true_up.x(), *true_up.y(), *true_up.z(), 0.0,
            -*forward.x(), -*forward.y(), -*forward.z(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        return orientation * Matrix4x4::translation(-*from.x(), -*from.y(), -*from.z());
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Self {mat: [
            self.get(0, 0) * rhs.get(0, 0) + self.get(0, 1) * rhs.get(1, 0) + self.get(0, 2) * rhs.get(2, 0) + self.get(0, 3) * rhs.get(3, 0),
            self.get(0, 0) * rhs.get(0, 1) + self.get(0, 1) * rhs.get(1, 1) + self.get(0, 2) * rhs.get(2, 1) + self.get(0, 3) * rhs.get(3, 1),
            self.get(0, 0) * rhs.get(0, 2) + self.get(0, 1) * rhs.get(1, 2) + self.get(0, 2) * rhs.get(2, 2) + self.get(0, 3) * rhs.get(3, 2),
            self.get(0, 0) * rhs.get(0, 3) + self.get(0, 1) * rhs.get(1, 3) + self.get(0, 2) * rhs.get(2, 3) + self.get(0, 3) * rhs.get(3, 3),
            self.get(1, 0) * rhs.get(0, 0) + self.get(1, 1) * rhs.get(1, 0) + self.get(1, 2) * rhs.get(2, 0) + self.get(1, 3) * rhs.get(3, 0),
            self.get(1, 0) * rhs.get(0, 1) + self.get(1, 1) * rhs.get(1, 1) + self.get(1, 2) * rhs.get(2, 1) + self.get(1, 3) * rhs.get(3, 1),
            self.get(1, 0) * rhs.get(0, 2) + self.get(1, 1) * rhs.get(1, 2) + self.get(1, 2) * rhs.get(2, 2) + self.get(1, 3) * rhs.get(3, 2),
            self.get(1, 0) * rhs.get(0, 3) + self.get(1, 1) * rhs.get(1, 3) + self.get(1, 2) * rhs.get(2, 3) + self.get(1, 3) * rhs.get(3, 3),
            self.get(2, 0) * rhs.get(0, 0) + self.get(2, 1) * rhs.get(1, 0) + self.get(2, 2) * rhs.get(2, 0) + self.get(2, 3) * rhs.get(3, 0),
            self.get(2, 0) * rhs.get(0, 1) + self.get(2, 1) * rhs.get(1, 1) + self.get(2, 2) * rhs.get(2, 1) + self.get(2, 3) * rhs.get(3, 1),
            self.get(2, 0) * rhs.get(0, 2) + self.get(2, 1) * rhs.get(1, 2) + self.get(2, 2) * rhs.get(2, 2) + self.get(2, 3) * rhs.get(3, 2),
            self.get(2, 0) * rhs.get(0, 3) + self.get(2, 1) * rhs.get(1, 3) + self.get(2, 2) * rhs.get(2, 3) + self.get(2, 3) * rhs.get(3, 3),
            self.get(3, 0) * rhs.get(0, 0) + self.get(3, 1) * rhs.get(1, 0) + self.get(3, 2) * rhs.get(2, 0) + self.get(3, 3) * rhs.get(3, 0),
            self.get(3, 0) * rhs.get(0, 1) + self.get(3, 1) * rhs.get(1, 1) + self.get(3, 2) * rhs.get(2, 1) + self.get(3, 3) * rhs.get(3, 1),
            self.get(3, 0) * rhs.get(0, 2) + self.get(3, 1) * rhs.get(1, 2) + self.get(3, 2) * rhs.get(2, 2) + self.get(3, 3) * rhs.get(3, 2),
            self.get(3, 0) * rhs.get(0, 3) + self.get(3, 1) * rhs.get(1, 3) + self.get(3, 2) * rhs.get(2, 3) + self.get(3, 3) * rhs.get(3, 3),
        ]};
    }
}

impl Mul<Vec4> for Matrix4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        return Vec4::raw(
            self.get(0, 0) * rhs.x() + self.get(0, 1) * rhs.y() + self.get(0, 2) * rhs.z() + self.get(0, 3) * rhs.w(), 
            self.get(1, 0) * rhs.x() + self.get(1, 1) * rhs.y() + self.get(1, 2) * rhs.z() + self.get(1, 3) * rhs.w(), 
            self.get(2, 0) * rhs.x() + self.get(2, 1) * rhs.y() + self.get(2, 2) * rhs.z() + self.get(2, 3) * rhs.w(), 
            self.get(3, 0) * rhs.x() + self.get(3, 1) * rhs.y() + self.get(3, 2) * rhs.z() + self.get(3, 3) * rhs.w(), 
        );
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Self;

    fn mul(self, scaler: f32) -> Self::Output {
        return Matrix4x4::new([
           self.get(0, 0) * scaler, self.get(0, 1) * scaler, self.get(0, 2) * scaler, self.get(0, 3) * scaler,
           self.get(1, 0) * scaler, self.get(1, 1) * scaler, self.get(1, 2) * scaler, self.get(1, 3) * scaler,
           self.get(2, 0) * scaler, self.get(2, 1) * scaler, self.get(2, 2) * scaler, self.get(2, 3) * scaler,
           self.get(3, 0) * scaler, self.get(3, 1) * scaler, self.get(3, 2) * scaler, self.get(3, 3) * scaler,
        ]);
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        return 
            util::equals_f32(&self.mat[0], &other.mat[0]) &&
            util::equals_f32(&self.mat[1], &other.mat[1]) &&
            util::equals_f32(&self.mat[2], &other.mat[2]) &&
            util::equals_f32(&self.mat[3], &other.mat[3]) &&
            util::equals_f32(&self.mat[4], &other.mat[4]) &&
            util::equals_f32(&self.mat[5], &other.mat[5]) &&
            util::equals_f32(&self.mat[6], &other.mat[6]) &&
            util::equals_f32(&self.mat[7], &other.mat[7]) &&
            util::equals_f32(&self.mat[8], &other.mat[8]) &&
            util::equals_f32(&self.mat[9], &other.mat[9]) &&
            util::equals_f32(&self.mat[10], &other.mat[10]) &&
            util::equals_f32(&self.mat[11], &other.mat[11]) &&
            util::equals_f32(&self.mat[12], &other.mat[12]) &&
            util::equals_f32(&self.mat[13], &other.mat[13]) &&
            util::equals_f32(&self.mat[14], &other.mat[14]) &&
            util::equals_f32(&self.mat[15], &other.mat[15]);
    }

    fn ne(&self, other: &Self) -> bool {
        return 
            !util::equals_f32(&self.mat[0], &other.mat[0]) ||
            !util::equals_f32(&self.mat[1], &other.mat[1]) ||
            !util::equals_f32(&self.mat[2], &other.mat[2]) ||
            !util::equals_f32(&self.mat[3], &other.mat[3]) ||
            !util::equals_f32(&self.mat[4], &other.mat[4]) ||
            !util::equals_f32(&self.mat[5], &other.mat[5]) ||
            !util::equals_f32(&self.mat[6], &other.mat[6]) ||
            !util::equals_f32(&self.mat[7], &other.mat[7]) ||
            !util::equals_f32(&self.mat[8], &other.mat[8]) ||
            !util::equals_f32(&self.mat[9], &other.mat[9]) ||
            !util::equals_f32(&self.mat[10], &other.mat[10]) ||
            !util::equals_f32(&self.mat[11], &other.mat[11]) ||
            !util::equals_f32(&self.mat[12], &other.mat[12]) ||
            !util::equals_f32(&self.mat[13], &other.mat[13]) ||
            !util::equals_f32(&self.mat[14], &other.mat[14]) ||
            !util::equals_f32(&self.mat[15], &other.mat[15]);
    }
}