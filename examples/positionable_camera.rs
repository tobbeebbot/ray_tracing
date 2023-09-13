use ray_tracing::material::Material::*;
use ray_tracing::hittable::*;
use ray_tracing::camera::CameraBuilder;
use glam::vec3;

fn main() {
    // World
    let mut world: Vec<Shape> = Vec::new();

    // Materials
    let material_ground = Lambertian(vec3(0.8, 0.8, 0.0));
    let material_center = Lambertian(vec3(0.1, 0.2, 0.5));
    let material_left =  Dielectric(1.5);
    let material_right  = Metal(vec3(0.8, 0.6, 0.2), 0.0);

    world.push(Shape::new_sphere(vec3( 0.0, -100.5, -1.0), 100.0, &material_ground));
    world.push(Shape::new_sphere(vec3( 0.0,    0.0, -1.0),   0.5, &material_center));
    world.push(Shape::new_sphere(vec3(-1.0,    0.0, -1.0),   0.5, &material_left));
    world.push(Shape::new_sphere(vec3(-1.0,    0.0, -1.0),  -0.4, &material_left));
    world.push(Shape::new_sphere(vec3( 1.0,    0.0, -1.0),   0.5, &material_right));

    let camera = CameraBuilder::default()
        .set_samples_per_pixel(200)
        .set_max_depth(64)
        .set_view_direction(vec3(-2.0, 2.0, 1.0), vec3(0.0, 0.0, -1.0))
        .set_vfov(20.0)
        .set_focus(11.0, 3.4)
        .build();

    camera.render(world);
}