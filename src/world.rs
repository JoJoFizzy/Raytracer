use crate::color::Color;
use crate::geometry::{Matrix4x4, Vec4};
use crate::intersection::{Comp, Intersection};
use crate::material::Material;
use crate::light::Light;
use crate::ray::Ray;
use crate::shape::{Shape, Sphere};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        return Self {
            objects: Vec::new(),
            lights: Vec::new(),
        };
    }

    pub fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = Vec::new();

        for shape in &self.objects {
            let inter = Intersection::intersect(&**shape, ray);
            xs.extend(inter);
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        return xs;
    }

    pub fn add_object(&mut self, shape: Box<dyn Shape>) {
        self.objects.push(shape);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn is_shadowed(&self, point: &Vec4) -> bool {
        for light in &self.lights {
            let v = light.position - *point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let ray = Ray::new(*point, direction);
            let mut inter = self.intersect_world(ray);

            if let Some(hit) = Intersection::hit(&mut inter) {
                if hit.t < distance {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn color_at(&self, ray: Ray, remaining: u32) -> Color {
        let mut intersection = self.intersect_world(ray);
        let xs = intersection.clone();
        
        if let Some(hit) = Intersection::hit(&mut intersection) {
            let comp = hit.prepare_computations(&ray, Some(&xs));
            return self.shade_hit(&comp, remaining);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    pub fn reflected_color(&self, comp: &Comp, remaining: u32) -> Color {
        if comp.object.material().reflective == 0.0 || remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comp.over_point, comp.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);

        return color * comp.object.material().reflective;
    }

    pub fn refracted_color(&self, comp: &Comp, remaining: u32) -> Color {
        if comp.object.material().transparency == 0.0 || remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let n_ratio = comp.n1 / comp.n2;
        let cos_i = comp.eyev.dot(&comp.normalv);
        let sin2_t = n_ratio*n_ratio * (1.0 - cos_i*cos_i);

        // Total Internal Reflection
        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let cos_t = f32::sqrt(1.0 - sin2_t);
        let direction = comp.normalv * (n_ratio * cos_i - cos_t) - comp.eyev * n_ratio;
        let refract_ray = Ray::new(comp.under_point, direction);
        let color = self.color_at(refract_ray, remaining - 1) * comp.object.material().transparency;

        return color;
    }

    pub fn shade_hit(&self, comp: &Comp, remaining: u32) -> Color {
        let shadowed: bool;

        if comp.object.material().transparency >= 1.0 {
            shadowed = false;
        } else {
            shadowed = self.is_shadowed(&comp.over_point);
        }

        let mut color = Color::new(0.0, 0.0, 0.0);

        for light in &self.lights {
            let c = comp
                .object
                .material()
                .lighting(comp.object, light, &comp.over_point, &comp.eyev, &comp.normalv, shadowed);

            color = color + c;
        }

        let reflected = self.reflected_color(comp, remaining);
        let refracted = self.refracted_color(comp, remaining);

        let material = comp.object.material();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comp.schlick();
            return color + reflected * reflectance + refracted * (1.0 - reflectance);
        } else {
            return color + reflected + refracted;
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut objects: Vec<Box<dyn Shape>> = Vec::new();
        let mut lights: Vec<Light> = Vec::new();

        let light = Light::point_light(Vec4::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        lights.push(light);

        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, None);
        let sphere1 = Sphere::new(material);

        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, None);
        let mut sphere2 = Sphere::new(material);
        sphere2.set_transform(Matrix4x4::scale(0.5, 0.5, 0.5));

        objects.push(Box::new(sphere1));
        objects.push(Box::new(sphere2));

        return Self {
            objects,
            lights,
        };
    }
}