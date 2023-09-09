use glam::vec3;
use ray_tracing::camera::Camera;
use ray_tracing::hittable::*;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    
    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(vec3(0.0, 0.5, -1.0), 0.2)));
    world.push(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(image_width, aspect_ratio);

    camera.render(world);
}
