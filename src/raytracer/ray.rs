#![deny(clippy::all)]
#![forbid(unsafe_code)]

use super::vec3::Vec3;
use Vec3 as Point3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}
