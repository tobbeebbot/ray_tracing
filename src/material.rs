use glam::{vec3, Vec3};
use rand::prelude::*;

use crate::{ray::Ray, hittable::HitRecord, camera::Color};

pub enum Material {
    Lambertian(Color), // color = "albedo"
    Metal(Color, f32), // albedo, fuzz
    Dielectric(f32) // index of refraction
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color) > {
        use Material::*;
        match &self {
            Lambertian(albedo) => {
                let mut scatter_direction = hit_record.normal + random_unit_vector();

                // To protect from degenerate (near zero) scatter directions
                if scatter_direction.length() <= 1e-8 {
                    scatter_direction = hit_record.normal;
                }

                let scattered_ray = Ray::new(hit_record.point, scatter_direction);

                let attenuation = *albedo;
                Some((scattered_ray, attenuation))
            },
            Metal(albedo, fuzz) => {
                let reflection_dir = reflect(ray.dir.normalize(), hit_record.normal);
                let scattered_ray = Ray::new(hit_record.point, reflection_dir + *fuzz * random_unit_vector());
                
                if scattered_ray.dir.dot(hit_record.normal) > 0.0 { 
                    Some((scattered_ray, *albedo))
                } else {
                    None // fuzz may result in invalid rays inside of sphere
                }
            },
            Dielectric(ir) => {
                let attenuation = Color::new(1.0, 1.0, 1.0);

                let refraction_ratio = if hit_record.front_face {
                    1.0 / *ir
                } else {
                    *ir
                };

                let unit_direction = ray.dir.normalize();

                let normal_dot_uiv = (-unit_direction).dot(hit_record.normal);
                let cos_theta = if normal_dot_uiv > 1.0 {
                    1.0
                } else {
                    normal_dot_uiv
                };
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let schlick_reflect = reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f32>();

                let direction = if cannot_refract || schlick_reflect {
                    reflect(unit_direction, hit_record.normal)
                } else {
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                let scattered_ray = Ray::new(hit_record.point, direction);

                Some((scattered_ray, attenuation))
            }
        }
    }
}

fn random_unit_vector() -> Vec3 {
   random_in_unit_sphere().normalize()
}

// fn random_on_hemisphere(normal: Vec3) -> Vec3 {
//     let on_unit_sphere = random_unit_vector();
//     if on_unit_sphere.dot(normal) > 0.0 {
//         on_unit_sphere
//     } else {
//         -on_unit_sphere
//     }
// }

pub fn random_vec() -> Vec3 {
    let mut rng = rand::thread_rng();
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0))
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec = random_vec();

        if vec.length_squared() < 1.0 {
            break vec;
        }
    }
}

fn reflect(in_dir: Vec3, normal: Vec3) -> Vec3 {
    in_dir - 2.0 * in_dir.dot(normal) * normal
}

fn refract(unit_in_vector: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let normal_dot_uiv = (-unit_in_vector).dot(normal);
    let cos_theta = if normal_dot_uiv > 1.0 {
        1.0
    } else {
        normal_dot_uiv
    };
    
    let r_out_perp = etai_over_etat * (unit_in_vector + (normal * cos_theta));
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}