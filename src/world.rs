use glam::vec3;
use itertools::Itertools;
use serde::Deserialize;

use crate::{hittable::Shape, material::Material};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum WorldMaterial {
    Lambertian {
        r: f32,
        g: f32,
        b: f32,
    }, // color = "albedo"
    Metal {
        r: f32,
        g: f32,
        b: f32,
        fuzz: f32,
    }, // albedo, fuzz
    Dielectric {
        refraction: f32
    } // index of refraction
}

impl Into<Material> for WorldMaterial {
    fn into(self) -> Material {
        match self {
            Self::Lambertian { r, g, b } => Material::Lambertian(vec3(r, g, b)),
            Self::Metal { r, g, b, fuzz } => Material::Metal(vec3(r, g, b), fuzz),
            Self::Dielectric { refraction } => Material::Dielectric(refraction),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorldSphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    material: WorldMaterial,
}

impl Into<Shape> for WorldSphere {
    fn into(self) -> Shape {
        Shape::new_sphere(vec3(self.x, self.y, self.z), self.radius, &self.material.into())
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct World {
    objects: Vec<WorldSphere>,
}

impl Into<Vec<Shape>> for World {
    fn into(self) -> Vec<Shape> {
        self.objects.into_iter().map(|shape| shape.into()).collect_vec()
    }
}

impl World {
    pub fn add(&mut self, sphere: WorldSphere) -> () {
        self.objects.push(sphere)
    }
}

