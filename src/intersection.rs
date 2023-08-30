use crate::geometry::Vec4;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::util;

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub object: &'a dyn Shape,
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a dyn Shape, t: f32) -> Self {
        return Self {
            object,
            t,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn from_uv(object: &'a dyn Shape, t: f32, u: f32, v: f32) -> Self {
        return Self {
            object,
            t,
            u,
            v,
        };
    }

    pub fn intersect(shape: &'a dyn Shape, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(shape.transform().invert());
        return shape.local_intersect(&ray);
    }

    pub fn hit(inter: &'a mut Vec<Intersection>) -> Option<Intersection<'a>> {
        inter.retain(|x| x.t > 0.0);
        inter.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        if inter.len() < 1 {
            return None;
        }

        return Some(inter[0]);
    }

    pub fn prepare_computations(&self, ray: &Ray, xs: Option<&Vec<Intersection>>) -> Comp {
        let mut n1: f32 = 1.0;
        let mut n2: f32 = 1.0;

        let mut stack: Vec<&dyn Shape> = Vec::new();
    
        if let Some(xs) = xs {
            for inter in xs {
                if stack.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = stack.last().unwrap().material().refraction;
                }

                let mut object_in_stack = false;
                for i in 0..stack.len() {
                    if stack[i].id() == inter.object.id() {
                        object_in_stack = true;
                        stack.remove(i);
                        break;
                    }
                }

                if !object_in_stack {
                    stack.push(inter.object);
                }

                if stack.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = stack.last().unwrap().material().refraction;
                }

                if self.object.id() == inter.object.id() && self.t == inter.t {
                    break;
                }
            }
        }

        let normalv = self.object.world_normal_at(&ray.at(self.t), xs.unwrap()[0]);

        return Comp::new(
            self.t,
            self.object,
            ray.at(self.t),
            -(ray.direction),
            normalv,
            ray.reflect(&normalv),
            n1,
            n2,
        );
    }
}

pub struct Comp<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
    pub point: Vec4,
    pub eyev: Vec4,
    pub normalv: Vec4,
    pub reflectv: Vec4,
    pub n1: f32, 
    pub n2: f32,
    pub inside: bool,
    pub over_point: Vec4,
    pub under_point: Vec4,
}

impl<'a> Comp<'a> {
    pub fn new(t: f32, object: &'a dyn Shape, point: Vec4, eyev: Vec4, normalv: Vec4, reflectv: Vec4, n1: f32, n2: f32) -> Self {
        let mut inside = false;
        let mut normalv = normalv;
        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * util::THRESHOLD_F32;
        let under_point = point - normalv * util::THRESHOLD_F32;

        return Self {
            t,
            object,
            point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            inside,
            over_point,
            under_point,
        };
    }

    pub fn schlick(&self) -> f32 {
        let mut cos = self.eyev.dot(&self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n*n * (1.0 - cos*cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }
}