use std::fs::File;
use std::io::Write;
use glam::{Vec3, vec3};
use indicatif::ProgressIterator;
use itertools::{self, Itertools};

type Color = Vec3;
fn stringify_color(color: Color) -> String {
    format!("{} {} {}", 
        color.x * 255.99, 
        color.y * 255.99,
        color.z * 255.99)
}
type Point3 = Vec3;
struct Ray {
    pub dir: Vec3,
    pub orig: Point3,
}

impl Ray {
    fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> Option<f32> {
    let oc = ray.orig - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant < 0.0 {
        None
    } else {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        Some(t)
    }
}

fn ray_color(ray: Ray) -> Color {
    let circle_center = vec3(0.0, 0.0, -1.0);
    if let Some(t) = hit_sphere(circle_center, 0.5, &ray)
    {

        let normal = ray.at(t) - circle_center;
        let normal = normal.normalize();
        return 0.5 * vec3(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = ray.dir.normalize();
    let a = 0.5*(unit_direction.y + 1.0);
    vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), a)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    
    // ensure image height is at least 1
    let image_height = ((image_width as f32) / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center : Point3 = vec3(0.0, 0.0, 0.0);
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3(viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    let pixel_colors =
        (0..image_height).cartesian_product(0..image_width)
        .progress_count(image_height as u64 * image_width as u64)
        .map(|(j, i)| {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray{orig: camera_center, dir: ray_direction};

            ray_color(ray)
        });

    // create the file
    let pixel_strings = pixel_colors.map(|pc| stringify_color(pc)).join("\n");
    let string_header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let file_content = string_header + &pixel_strings;
    
    File::create("image.ppm")
        .expect("Should be able to create a new file.")
        .write_all(file_content.as_bytes())
        .expect("Should be able to write to it as well.");

}
