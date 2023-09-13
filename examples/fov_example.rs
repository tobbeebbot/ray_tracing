use std::f32::consts::PI;

use glam::vec3;
use ray_tracing::camera::{Color, CameraBuilder};
use ray_tracing::hittable::*;
use ray_tracing::material::Material::*;

fn main() {
    // Materials
    let material_left  = Lambertian(Color::new(0.0,0.0,1.0));
    let material_right = Lambertian(Color::new(1.0,0.0,0.0));


    // World
    let mut world: Vec<Shape> = Vec::new();
    let r = (PI / 4.0).cos();


    world.push(Shape::new_sphere(vec3(-r, 0.0, -1.0), r, &material_left));
    world.push(Shape::new_sphere(vec3( r, 0.0, -1.0), r, &material_right));


    let camera = CameraBuilder::default().build();

    camera.render(world);
}
