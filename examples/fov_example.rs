use std::f32::consts::PI;
use std::rc::Rc;

use glam::vec3;
use ray_tracing::camera::{Color, CameraBuilder};
use ray_tracing::hittable::*;
use ray_tracing::material::Material::*;

fn main() {
    // Materials
    let material_left  = Rc::new(Lambertian(Color::new(0.0,0.0,1.0)));
    let material_right = Rc::new(Lambertian(Color::new(1.0,0.0,0.0)));


    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let r = (PI / 4.0).cos();


    world.push(Box::new(Sphere::new(vec3(-r, 0.0, -1.0), r, material_left)));
    world.push(Box::new(Sphere::new(vec3( r, 0.0, -1.0), r, material_right)));


    let camera = CameraBuilder::default().build();

    camera.render(world);
}
