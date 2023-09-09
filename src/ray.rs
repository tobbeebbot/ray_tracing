use glam::Vec3;

pub type Point = Vec3;
pub struct Ray {
    pub dir: Vec3,
    pub orig: Point,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.orig + self.dir * t
    }
}