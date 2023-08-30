use tracer::camera::Camera;
use tracer::color::Color;
use tracer::geometry::{Matrix4x4, Vec4};
use tracer::material::Material;
use tracer::model::Model;
use tracer::light::Light;
use tracer::pattern::CheckeredPattern;
use tracer::shape::{Cube, Plane};
use tracer::view::View;
use tracer::world::World;

fn main() {
    let mut world = World::default();
    world.objects.pop();
    world.objects.pop();
    world.lights.pop();

    let light = Light::point_light(Vec4::point(0.0, 20.0, 3.0), Color::new(1.0, 1.0, 1.0));
    world.add_light(light);

    let material = Material::default();
    let mut model = Model::new(material, "obj_files/obj_african_head.obj");
    model.material.ambient = 0.8;
    model.transform = Matrix4x4::translation(0.0, 1.0, 5.0) 
        * Matrix4x4::rotatation_y(std::f32::consts::PI / 2.0) 
        * Matrix4x4::rotatation_x(-std::f32::consts::PI / 4.0) 
        * Matrix4x4::scale(10.0, 10.0, 10.0);
    world.add_object(Box::new(model));

    let mut material = Material::default();
    material.pattern = Some(Box::new(CheckeredPattern::default()));
    let water = Plane::new(material);
    world.add_object(Box::new(water));

    let material = Material::default();
    let mut beach = Cube::new(material);
    beach.transform = Matrix4x4::scale(5.0, 1.0, 1.0) * Matrix4x4::translation(0.0, 1.0, -8.5);
    world.add_object(Box::new(beach));

    let mut camera = Camera::new(300.0, 150.0, std::f32::consts::PI/3.0);
    let from = Vec4::point(0.0, 3.0, -10.0);
    let to = Vec4::point(0.0, 5.5, 0.0);
    let up = Vec4::vector(0.0, 0.0, -1.0);
    camera.set_view_transform(from, to, up);

    let canvas = camera.render(&world);
    let mut view = View::new(canvas);
    view.run();
}
