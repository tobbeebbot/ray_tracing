use glam::Vec3;
use crate::ray::{Point, Ray};
use crate::interval::Interval;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn new_from_ray(out_normal: Vec3, t: f32, ray: &Ray) -> HitRecord {
        let front_face =  ray.dir.dot(out_normal) < 0.0;
        let normal = if front_face { out_normal } else { -out_normal };
        HitRecord { point: ray.at(t), normal: normal, t: t, front_face: front_face }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let t =
                interval.surround_where((-half_b - sqrtd) / a)
                .or((interval).surround_where((-half_b + sqrtd) / a))?;

            let hit_point = ray.at(t);
            let outward_normal = (hit_point - self.center) / self.radius;
            Some(HitRecord::new_from_ray(outward_normal, t, &ray))
        }
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}