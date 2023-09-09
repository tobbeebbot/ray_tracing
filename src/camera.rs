use crate::ray::{Ray, Point};
use crate::hittable::Hittable;
use glam::{vec3, Vec3};
use indicatif::ProgressIterator;
use std::f32::INFINITY;
use std::io::Write;
use itertools::{self, Itertools};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}
type Color = Vec3;
fn stringify_color(color: &Color) -> String {
    format!(
        "{} {} {}",
        color.x * 255.99,
        color.y * 255.99,
        color.z * 255.99
    )
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f32) -> Camera {
        // ensure image height is at least 1
        let image_height = ((image_width as f32) / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };
    
        // camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let camera_center : Point = vec3(0.0, 0.0, 0.0);
        
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3(viewport_width, 0.0, 0.0);
        let viewport_v = vec3(0.0, -viewport_height, 0.0);
    
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
    
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v
        }
    }

    pub fn render(&self, world: Vec<Box<dyn Hittable>>) -> () {
        let pixel_colors = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .map(|(j, i)| {
                let pixel_center =
                    self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    orig: self.center,
                    dir: ray_direction,
                };

                Self::ray_color(&ray, &world)
            });
        
        // create the file
        let pixel_strings = pixel_colors.map(|pc| stringify_color(&pc)).join("\n");
        let string_header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let file_content = string_header + &pixel_strings;
        
        std::fs::File::create("image.ppm")
        .expect("Should be able to create a new file.")
        .write_all(file_content.as_bytes())
        .expect("Should be able to write to it as well.");
    }

    fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Color {
        let ray_trace = world.iter().fold(None, |acc, elem| 
            match acc {
                None => elem.hit(&ray, 0.0..INFINITY),
                Some(hr) => elem.hit(&ray, 0.0..hr.t).or(Some(hr)),
            });

        if let Some(hit_record) = ray_trace {
            let normal = hit_record.normal;
            return 0.5 * vec3(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
        }

        let unit_direction = ray.dir.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), a)
    }
}
