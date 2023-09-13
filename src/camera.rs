use crate::interval::Interval;
use crate::ray::{Ray, Point};
use crate::hittable::{Hittable, Shape};
use glam::{vec3, Vec3};
use indicatif::ParallelProgressIterator;
use std::f32::INFINITY;
use std::io::Write;
use itertools::{self, Itertools};
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Copy, Clone)]
pub struct CameraBuilder {
    vfov: f32,  // Vertical view angle (field of view)
    samples_per_pixel: u32,
    max_depth: u32,

    look_from: Point,  // Point camera is looking from
    look_at: Point,   // Point camera is looking at
    vup: Vec3,     // Camera-relative "up" direction

    image_width: u32,
    aspect_ratio: f32,

    defocus_angle: f32,
    focus_dist: f32,
}

impl CameraBuilder {
    pub fn default() -> CameraBuilder {
        // camera
        let vfov = 90.0;  // Vertical view angle (field of view)
        let look_from = Point::new(0.0, 0.0, 0.0);  // Point camera is looking from
        let look_at   = Point::new(0.0, 0.0, -1.0);   // Point camera is looking at
        let vup      = vec3(0.0, 1.0, 0.0);     // Camera-relative "up" direction
        
        let max_depth = 64;
        let samples_per_pixel = 64;
    
        let image_width = 400;
        let aspect_ratio = 16.0 / 9.0;

        let defocus_angle = 0.0;
        let focus_dist = 1.0;
    
        CameraBuilder { vfov, samples_per_pixel, max_depth, look_from, look_at, vup, image_width, aspect_ratio, defocus_angle, focus_dist }
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) -> CameraBuilder {
        self.aspect_ratio = ratio;
        self.clone()
    }

    pub fn set_image_width(&mut self, width: u32) -> CameraBuilder {
        self.image_width = width;
        self.clone()
    }

    pub fn set_max_depth(&mut self, max_depth: u32) -> CameraBuilder {
        self.max_depth = max_depth;
        self.clone()
    }

    pub fn set_samples_per_pixel(&mut self, samples: u32) -> CameraBuilder {
        self.samples_per_pixel = samples;
        self.clone()
    }

    pub fn set_view_direction(&mut self, look_from: Point, look_at: Point) -> CameraBuilder {
        self.look_from = look_from;
        self.look_at = look_at;
        self.clone()
    }

    pub fn set_vfov(&mut self, vfov: f32) -> CameraBuilder {
        self.vfov = vfov;
        self.clone()
    }

    pub fn set_focus(&mut self, defocus_angle:f32, focus_dist:f32) -> CameraBuilder {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;
        self.clone()
    }

    pub fn build(&self) -> Camera {
        Camera::new(
            self.image_width,
            self.aspect_ratio,
            self.vfov,
            self.samples_per_pixel,
            self.max_depth,
            self.look_from,
            self.look_at,
            self.vup,
            self.defocus_angle,
            self.focus_dist)
    }
}

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub defocus_angle: f32,
}
pub type Color = Vec3;
fn stringify_color(color: &Color) -> String {
    format!(
        "{} {} {}",
        color.x * 255.99,
        color.y * 255.99,
        color.z * 255.99
    )
}
fn linnear_to_gamma(color: &Color) -> Color {
    Color { x: color.x.sqrt(), y: color.y.sqrt(), z: color.z.sqrt() }
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        vfov: f32,
        samples_per_pixel: u32,
        max_depth: u32,
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        defocus_angle: f32,
        focus_dist: f32) -> Camera
    {
        // ensure image height is at least 1
        let image_height = ((image_width as f32) / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };
    
        let theta = Self::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let center = look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;   // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge
    
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
    
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (Self::degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
    
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    pub fn render(&self, world: Vec<Shape>) -> () {
        let pixel_colors = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .map(|(j, i)| {
                let pixel_sum = (0..self.samples_per_pixel)
                .map(|_| self.get_ray(i, j))
                .map(|ray| self.ray_color(&ray, self.max_depth, &world))
                .sum::<Color>();
            pixel_sum / self.samples_per_pixel as f32
            })
            .collect::<Vec<Color>>();
        
        // create the file
        let pixel_strings = pixel_colors.iter()
            .map(|pc| linnear_to_gamma(pc))
            .map(|pc| stringify_color(&pc))
            .join("\n");
        let string_header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let file_content = string_header + &pixel_strings;
        
        std::fs::File::create("image.ppm")
        .expect("Should be able to create a new file.")
        .write_all(file_content.as_bytes())
        .expect("Should be able to write to it as well.");
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &Vec<Shape>) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }
        let ray_trace = world.iter().fold(None, |acc, elem| 
            match acc {
                None => elem.hit(&ray, Interval::new(0.001, INFINITY)),
                Some((hr, material)) => elem.hit(&ray, Interval::new(0.0, hr.t)).or(Some((hr, material))),
            });

        if let Some((hit_record, material)) = ray_trace {
            if let Some((scattered_ray, attenuation)) = material.scatter(ray, &hit_record) {
                return attenuation * self.ray_color(&scattered_ray, depth - 1, world);
            } else {
                // Not getting a scatter back is absorbtion
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        // background
        let unit_direction = ray.dir.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), a)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center =
                    self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray {
            orig: ray_origin,
            dir: ray_direction,
        }
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f32>();
        let py = -0.5 + rng.gen::<f32>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Self::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = vec3(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }


    
}
