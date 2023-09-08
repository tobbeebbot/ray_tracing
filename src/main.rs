use std::{fs::File, f32::INFINITY};
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

struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new_from_ray(out_normal: Vec3, t: f32, ray: &Ray) -> HitRecord {
        let front_face =  ray.dir.dot(out_normal) < 0.0;
        let normal = if front_face { out_normal } else { -out_normal };
        HitRecord { point: ray.at(t), normal: normal, t: t, front_face: front_face }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let small_root = (-half_b - sqrtd) / a;
            let root = if small_root <= t_min || small_root >= t_max
            {
                let big_root = (-half_b + sqrtd) / a;
                if big_root <= t_min || big_root >= t_max {
                    None
                } else {
                    Some(big_root)
                }
            } else {
                Some(small_root)
            };

            let t = root?;
            let hit_point = ray.at(t);
            let outward_normal = (hit_point - self.center) / self.radius;
            Some(HitRecord::new_from_ray(outward_normal, t, &ray))
        }
    }
}

fn ray_color(ray: Ray, world: &Vec<Box<dyn Hittable>>) -> Color {

    if let Some(hr) = world.iter()
        .fold(None, |acc, elem| {
            match acc {
                None => elem.hit(&ray, 0.0, INFINITY),
                Some(hr) => elem.hit(&ray, 0.0, hr.t).or(Some(hr))
            }
        })
    {
        let normal = hr.normal;
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

    // World

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere{ center: vec3(0.0, 0.0, -1.0), radius: 0.5}));
    world.push(Box::new(Sphere{ center: vec3(0.0, 0.5, -1.0), radius: 0.2}));
    world.push(Box::new(Sphere{ center: vec3(0.0, -100.5, -1.0), radius: 100.0}));

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

            ray_color(ray, &world)
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
