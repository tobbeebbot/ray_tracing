use std::rc::Rc;

use ray_tracing::hittable::Hittable;
use ray_tracing::material::Material::*;
use ray_tracing::hittable::*;
use ray_tracing::camera::CameraBuilder;
use glam::vec3;

fn main() {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    // Materials
    let material_ground = Rc::new(Lambertian(vec3(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian(vec3(0.1, 0.2, 0.5)));
    let material_left =  Rc::new(Dielectric(1.5));
    let material_right  = Rc::new(Metal(vec3(0.8, 0.6, 0.2), 0.0));

    world.push(Box::new(Sphere::new(vec3( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(vec3( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.push(Box::new(Sphere::new(vec3(-1.0,    0.0, -1.0),   0.5, material_left.clone())));
    world.push(Box::new(Sphere::new(vec3(-1.0,    0.0, -1.0),  -0.4, material_left)));
    world.push(Box::new(Sphere::new(vec3( 1.0,    0.0, -1.0),   0.5, material_right)));

    let camera = CameraBuilder::default()
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_view_direction(vec3(-2.0, 2.0, 1.0), vec3(0.0, 0.0, -1.0))
        .set_vfov(20.0)
        .set_focus(11.0, 3.4)
        .build();

    // cam.samples_per_pixel = 100;
    // cam.max_depth         = 50;

    // cam.vfov     = 90;
    // cam.lookfrom = point3(-2,2,1);
    // cam.lookat   = point3(0,0,-1);
    // cam.vup      = vec3(0,1,0);

    camera.render(world);
}