use crate::color::Color;
use crate::geometry::Vec4;
use crate::light::Light;
use crate::pattern::Pattern;
use crate::shape::Shape;

pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refraction: f32,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32, reflective: f32, transparency: f32, refraction: f32, pattern: Option<Box<dyn Pattern>>) -> Self {
        return Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refraction,
            pattern,
        };
    }

    pub fn lighting(&self, object: &dyn Shape, light: &Light, point: &Vec4, eyev: &Vec4, normalv: &Vec4, in_shadow: bool) -> Color  {
        let mut color = self.color;

        if let Some(pattern) = &self.pattern {
            color = pattern.color_at_object(object, point);
        }

        let effective_color = color * light.intensity;
        let lightv = (light.position - *point).normalize();
        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let diffuse: Color;
        let specular: Color;

        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        return ambient + diffuse + specular;
    }
}

impl Default for Material {
    fn default() -> Self {
        return Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refraction: 1.0,
            pattern: None,
        }
    }
}