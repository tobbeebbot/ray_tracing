
use std::rc::Rc;

use rand::Rng;
use ray_tracing::hittable::Hittable;
use ray_tracing::material::{Material::*, random_vec};
use ray_tracing::hittable::*;
use ray_tracing::camera::CameraBuilder;
use glam::vec3;

fn main() {
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material = Rc::new(Lambertian(vec3(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(vec3(0.0,-1000.0,0.0), 1000.0, ground_material)));

    let mut rng = rand::thread_rng();    

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat = rng.gen::<f32>();
            let center = vec3(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9*rng.gen::<f32>());

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if (choose_mat < 0.8) {
                    // diffuse
                    let albedo = random_vec() * random_vec();
                    Lambertian(albedo)
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = random_vec();
                    let fuzz = rng.gen_range(0.0..0.5);
                    Metal(albedo, fuzz)
                } else {
                    // glass
                    Dielectric(1.5)
                };
                let sphere_material_p = Rc::new(sphere_material);
                world.push(Box::new(Sphere::new(center, 0.2, sphere_material_p.clone())));
            }
        }
    }

    let material1 = Rc::new(Dielectric(1.5));
    world.push(Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian(vec3(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal(vec3(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let camera = CameraBuilder::default()
        .set_image_width(1200)
        .set_samples_per_pixel(500)
        .set_max_depth(50)
        .set_vfov(20.0)
        .set_view_direction(vec3(13.0, 2.0, 3.0), vec3(0.0, 0.0, 0.0))
        .set_focus(0.6, 10.0)
        .build();

    camera.render(world);
}