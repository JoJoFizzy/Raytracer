use std::mem::swap;

use uuid::Uuid;
use crate::geometry::{Matrix4x4, Vec4};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::util;

pub trait Shape {
    fn id(&self) -> &Uuid;
    fn transform(&self) -> &Matrix4x4;
    fn set_transform(&mut self, matrix: Matrix4x4);
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: &Vec4, hit: Intersection) -> Vec4;
    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4;
}

pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix4x4,
    pub material: Material,
}

impl Sphere {
    pub fn new(material: Material) -> Self {
        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
        };
    }

    pub fn glass_sphere() -> Self {
        let mut material = Material::default();
        material.transparency = 1.0;
        material.refraction = 1.5;
        material.reflective = 0.8;
        material.specular = 1.0;
        material.shininess = 300.0;

        return Sphere::new(material);
    }
}

impl Shape for Sphere {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn transform(&self) -> &Matrix4x4 {
        return &self.transform;
    }

    fn set_transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn material_mut(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Vec4::point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b*b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let mut intersections: Vec<Intersection> = Vec::new();

        let result1 = (-b - discriminant.sqrt()) / (2.0 * a);
        intersections.push(Intersection::new(self, result1));

        let result2 = (-b + discriminant.sqrt()) / (2.0 * a);
        intersections.push(Intersection::new(self, result2));

        return intersections;
    }

    fn local_normal_at(&self, local_point: &Vec4, _: Intersection) -> Vec4 {
        let local_normal = *local_point - Vec4::point(0.0, 0.0, 0.0);
        return local_normal.normalize();
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}

pub struct Plane {
    pub id: Uuid,
    pub transform: Matrix4x4,
    pub material: Material,
}

impl Plane {
    pub fn new(material: Material) -> Self {
        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
        };
    }
}

impl Shape for Plane {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn transform(&self) -> &Matrix4x4 {
        return &self.transform;
    }

    fn set_transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn material_mut(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.y().abs() < util::THRESHOLD_F32 {
            return Vec::new();
        }

        let t = -ray.origin.y() / ray.direction.y();

        let mut result: Vec<Intersection> = Vec::new();
        result.push(Intersection::new(self, t));

        return result;
    }

    fn local_normal_at(&self, _: &Vec4, _: Intersection) -> Vec4 {
        return Vec4::vector(0.0, 1.0, 0.0);
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}  

pub struct Cube {
    pub id: Uuid,
    pub transform: Matrix4x4,
    pub material: Material,
}

impl Cube {
    pub fn new(material: Material) -> Self {
        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
        };
    }

    fn check_axis(origin: &f32, direction: &f32) -> (f32, f32) {
        let mut tmin: f32;
        let mut tmax: f32;

        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        if direction.abs() >= util::THRESHOLD_F32 {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f32::INFINITY;
            tmax = tmax_numerator * f32::INFINITY;
        }

        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }

        return (tmin, tmax);
    }
}

impl Shape for Cube {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn transform(&self) -> &Matrix4x4 {
        return &self.transform;
    }

    fn set_transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn material_mut(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());

        let tmin = util::max_f32(&vec![xtmin, ytmin, ztmin]).unwrap();
        let tmax = util::min_f32(&vec![xtmax, ytmax, ztmax]).unwrap();

        if tmin > tmax {
            return Vec::new();
        }

        return vec![Intersection::new(self, tmin), Intersection::new(self, tmax)];
    }

    fn local_normal_at(&self, local_point: &Vec4, _: Intersection) -> Vec4 {
        let maxc = util::max_f32(&vec![local_point.x().abs(), local_point.y().abs(), local_point.z().abs()]).unwrap();

        if maxc == local_point.x().abs() {
            return Vec4::vector(*local_point.x(), 0.0, 0.0);
        } else if maxc == local_point.y().abs() {
            return Vec4::vector(0.0, *local_point.y(), 0.0);
        }

        return Vec4::vector(0.0, 0.0, *local_point.z());
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}

pub struct Cylinder {
    pub id: Uuid,
    pub transform: Matrix4x4,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub closed: bool,
}

impl Cylinder {
    pub fn new(material: Material, minimum: f32, maximum: f32, closed: bool) -> Self {
        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
            minimum,
            maximum,
            closed,
        };
    }

    fn check_cap(ray: Ray, t: f32) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        return (x*x + z*z) <= 1.0;
    }

    pub fn intersect_caps(&self, ray: Ray) -> Vec<Intersection> {
        if !self.closed || util::equals_f32(ray.direction.y(), &0.0) {
            return Vec::new();
        }

        let mut xs: Vec<Intersection> = Vec::new();
        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            xs.push(Intersection::new(self, t));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            xs.push(Intersection::new(self, t));
        }

        return xs;
    }
}

impl Shape for Cylinder {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn transform(&self) -> &Matrix4x4 {
        return &self.transform;
    }

    fn set_transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn material_mut(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

        if util::equals_f32(&a, &0.0) {
            return Vec::new();
        }

        let b = 2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();
        let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
        let disc = b*b - 4.0 * a * c;

        if disc < 0.0 {
            return Vec::new();
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let mut xs: Vec<Intersection> = Vec::new();

        let y0 = ray.origin.y() + t0 * ray.direction.y();
        if self.minimum < y0  && y0 < self.maximum {
            xs.push(Intersection::new(self, t0));
        }

        let y1 = ray.origin.y() + t1 * ray.direction.y();
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(self, t1));
        }

        xs.append(&mut self.intersect_caps(*ray));

        return xs;
    }

    fn local_normal_at(&self, local_point: &Vec4, _: Intersection) -> Vec4 {
        let dist = local_point.x().powi(2) + local_point.z().powi(2);

        if dist < 1.0 && *local_point.y() >= self.maximum - util::THRESHOLD_F32 {
            return Vec4::vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && *local_point.y() >= self.maximum - util::THRESHOLD_F32 {
            return Vec4::vector(0.0, -1.0, 0.0);
        } else {
            return Vec4::vector(*local_point.x(), 0.0, *local_point.z());
        }
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}