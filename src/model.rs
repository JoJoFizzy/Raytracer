use std::fs::File;
use std::io::{BufReader, BufRead};
use uuid::Uuid;

use crate::geometry::{Matrix4x4, Vec4};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::util;

pub struct Triangle {
    id: Uuid,
    transform: Matrix4x4,
    material: Material,
    p1: Vec4,
    p2: Vec4,
    p3: Vec4,
    e1: Vec4,
    e2: Vec4,
    normal: Vec4,
}

impl Triangle {
    pub fn new(material: Material, p1: Vec4, p2: Vec4, p3: Vec4) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = (e2.cross(&e1)).normalize();

        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        };
    }
}

impl Shape for Triangle {
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
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);

        if det.abs() < util::THRESHOLD_F32 {
            return Vec::new();
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = p1_to_origin.dot(&dir_cross_e2) * f;

        if u < 0.0 || u > 1.0 {
            return Vec::new();
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = ray.direction.dot(&origin_cross_e1) * f;

        if v < 0.0 || (u + v) > 1.0 {
            return Vec::new();
        }

        let t = self.e2.dot(&origin_cross_e1) * f;

        return vec![Intersection::from_uv(self, t, u, v)];
    }

    fn local_normal_at(&self, _: &Vec4, _: Intersection) -> Vec4 {
        return self.normal;
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}

pub struct SmoothTriangle {
    id: Uuid,
    transform: Matrix4x4,
    material: Material,
    p1: Vec4,
    p2: Vec4,
    p3: Vec4,
    n1: Vec4,
    n2: Vec4,
    n3: Vec4,
}

impl SmoothTriangle {
    pub fn new(material: Material, p1: Vec4, p2: Vec4, p3: Vec4, n1: Vec4, n2: Vec4, n3: Vec4) -> Self {
        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material: material,
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
        };
    }
}

impl Shape for SmoothTriangle {
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
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;

        let dir_cross_e2 = ray.direction.cross(&e2);
        let det = e1.dot(&dir_cross_e2);

        if det.abs() < util::THRESHOLD_F32 {
            return Vec::new();
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = p1_to_origin.dot(&dir_cross_e2) * f;

        if u < 0.0 || u > 1.0 {
            return Vec::new();
        }

        let origin_cross_e1 = p1_to_origin.cross(&e1);
        let v = ray.direction.dot(&origin_cross_e1) * f;

        if v < 0.0 || (u + v) > 1.0 {
            return Vec::new();
        }

        let t = e2.dot(&origin_cross_e1) * f;

        return vec![Intersection::from_uv(self, t, u, v)];
    }

    fn local_normal_at(&self, _: &Vec4, hit: Intersection) -> Vec4 {
        return self.n2 * hit.u + self.n3 * hit.v + self.n1 * (1.0 - hit.u - hit.v);
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}

pub struct Model {
    pub id: Uuid,
    pub transform: Matrix4x4,
    pub material: Material,
    pub triangles: Vec<Box<dyn Shape>>,
}

impl Model {    
    pub fn new(material: Material, file_path: &str) -> Self {
        let triangles = Self::process_obj_file(&material, file_path);

        return Self {
            id: Uuid::new_v4(),
            transform: Matrix4x4::identity(),
            material,
            triangles,
        };
    }

    fn process_obj_file(material: &Material, file_path: &str) -> Vec<Box<dyn Shape>> {
        let mut verts: Vec<Vec4> = Vec::new();
        let mut vert_normals: Vec<Vec4> = Vec::new();
        let mut face_verts: Vec<Vec<usize>> = Vec::new();
        let mut face_normals: Vec<Vec<usize>> = Vec::new();

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let tokens: Vec<_> = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect();

            if let Some(first) = tokens.first() {
                if first == "v" {
                    let vertex = Vec4::point(
                        tokens[1].parse::<f32>().unwrap(), 
                        tokens[2].parse::<f32>().unwrap(), 
                        tokens[3].parse::<f32>().unwrap(),
                    );
                    verts.push(vertex);
                } else if first == "vn" {
                    let vnormal = Vec4::vector(
                        tokens[1].parse::<f32>().unwrap(), 
                        tokens[2].parse::<f32>().unwrap(), 
                        tokens[3].parse::<f32>().unwrap(),
                    );
                    vert_normals.push(vnormal);
                } else if first == "f" {
                    let body: Vec<_> = tokens[1..]
                        .join("/")
                        .split("/")
                        .map(|s| s.to_owned().parse::<usize>().unwrap() - 1)
                        .collect();

                    let face: Vec<usize> = Vec::from([
                        body[0],
                        body[3],
                        body[6],
                    ]);
                    face_verts.push(face);

                    let fnormal: Vec<usize> = Vec::from([
                        body[2],
                        body[5],
                        body[8],
                    ]);
                    face_normals.push(fnormal);
                }
            }
        }

        let mut triangles: Vec<Box<dyn Shape>> = Vec::new();

        if face_normals.len() > 0 {
            for i in 0..face_verts.len() {
                let face = &face_verts[i];
                let normal = &face_normals[i];

                // Not implementing Patterns right now for models
                let material = Material::new(
                    material.color, 
                    material.ambient, 
                    material.diffuse, 
                    material.specular, 
                    material.shininess, 
                    material.reflective, 
                    material.transparency, 
                    material.refraction, 
                    None,
                );

                let triangle = SmoothTriangle::new(
                    material,
                    verts[face[0]], 
                    verts[face[1]], 
                    verts[face[2]],
                    vert_normals[normal[0]], 
                    vert_normals[normal[1]], 
                    vert_normals[normal[2]],
                );

                triangles.push(Box::new(triangle));
            }            
        } else {
            for i in 0..face_verts.len() {
                let face = &face_verts[i];

                // Not implementing Patterns right now for models
                let material = Material::new(
                    material.color, 
                    material.ambient, 
                    material.diffuse, 
                    material.specular, 
                    material.shininess, 
                    material.reflective, 
                    material.transparency, 
                    material.refraction, 
                    None,
                );

                let triangle = Triangle::new(
                    material,
                    verts[face[0]], 
                    verts[face[1]], 
                    verts[face[2]],
                );

                triangles.push(Box::new(triangle));
            }
        }

        return triangles;
    }
}

impl Shape for Model {
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
        let mut xs: Vec<Intersection> = Vec::new();

        for tri in &self.triangles {
            xs.append(&mut tri.local_intersect(ray));
        }

        return xs;
    }

    fn local_normal_at(&self, local_point: &Vec4, _: Intersection) -> Vec4 {
        let ray = Ray::new(*local_point, Vec4::vector(0.0, 0.0, 0.0));

        for tri in &self.triangles {
            let inter = tri.local_intersect(&ray);
            if inter.len() > 0 {
                return tri.local_normal_at(&local_point, inter[0]);
            }
        }

        return Vec4::vector(0.0, 0.0, 0.0);
    }

    fn world_normal_at(&self, world_point: &Vec4, i: Intersection) -> Vec4 {
        let local_point = self.transform().invert() * *world_point;
        let local_normal = self.local_normal_at(&local_point, i);
        let world_normal = self.transform().invert().transpose() * local_normal;
        let world_normal = Vec4::vector(*world_normal.x(), *world_normal.y(), *world_normal.z());
    
        return world_normal.normalize();
    }
}