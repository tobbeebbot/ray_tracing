
use itertools::Itertools;
use rand::Rng;
use ray_tracing::material::{Material::*, random_vec};
use ray_tracing::hittable::*;
use ray_tracing::camera::CameraBuilder;
use glam::vec3;

fn main() {
    // World

    
    let mut world = (-11..11).cartesian_product(-11..11)
    .map(|(a, b)| {
        let a = a as f32;
        let b = b as f32;
        let center = vec3(a + 0.9 * rand::thread_rng().gen::<f32>(), 0.2, b + 0.9*rand::thread_rng().gen::<f32>());
        center
    })
    .filter(|&center| ( center - vec3(4.0, 0.2, 0.0)).length() > 0.9)
    .map(|center| {  
            let choose_mat = rand::thread_rng().gen::<f32>();
            let sphere_material = if choose_mat < 0.8 {
                // diffuse
                let albedo = (random_vec() * random_vec()).abs(); // OBS super important that all elements of the Color vec are positive
                Lambertian(albedo)
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_vec().abs();
                let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                Metal(albedo, fuzz)
            } else {
                // glass
                Dielectric(1.5)
            };
            Shape::new_sphere(center, 0.2, &sphere_material)
        })
        .collect::<Vec<Shape>>();
    
    let ground_material = Lambertian(vec3(0.5, 0.5, 0.5));
    world.push(Shape::new_sphere(vec3(0.0,-1000.0,0.0), 1000.0, &ground_material));

    let material1 = Dielectric(1.5);
    world.push(Shape::new_sphere(vec3(0.0, 1.0, 0.0), 1.0, &material1));

    let material2 = Lambertian(vec3(0.4, 0.2, 0.1));
    world.push(Shape::new_sphere(vec3(-4.0, 1.0, 0.0), 1.0, &material2));

    let material3 = Metal(vec3(0.7, 0.6, 0.5), 0.0);
    world.push(Shape::new_sphere(vec3(4.0, 1.0, 0.0), 1.0, &material3));

    let camera = CameraBuilder::default()
        .set_image_width(800)
        .set_samples_per_pixel(128)
        .set_max_depth(50)
        .set_vfov(20.0)
        .set_view_direction(vec3(13.0, 2.0, 3.0), vec3(0.0, 0.0, 0.0))
        .set_focus(0.6, 10.0)
        .build();

    camera.render(world);
}