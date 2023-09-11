use glam::vec3;
use ray_tracing::camera::CameraBuilder;
use ray_tracing::hittable::*;
use ray_tracing::material::Material::*;
use std::rc::Rc;

fn main() {
    // Materials
    let material_ground = Rc::new(Lambertian(vec3(0.8, 0.8, 0.0)));
    let material_center =  Rc::new(Dielectric(1.5));
    let material_left   = Rc::new(Metal(vec3(0.9, 0.9, 0.9), 0.1));
    let material_right  = Rc::new(Metal(vec3(0.2, 0.6, 0.8), 1.0));
    let material_behind  = Rc::new(Metal(vec3(0.1, 0.6, 0.2), 0.6));
    let material_right2  = Rc::new(Metal(vec3(0.8, 0.3, 0.1), 0.8));


    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(vec3( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(vec3( 0.0,    -0.3, -3.0),   0.2, material_behind))); // small one behind
    world.push(Box::new(Sphere::new(vec3( 0.0,    0.25, -1.0),   0.25, material_center)));
    world.push(Box::new(Sphere::new(vec3(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.push(Box::new(Sphere::new(vec3( 1.0,    0.0, -1.0),   0.5, material_right)));
    world.push(Box::new(Sphere::new(vec3( 0.2,    -0.4, -0.6),   0.1, material_right2)));


    let camera = CameraBuilder::default().build();
    camera.render(world);
}
