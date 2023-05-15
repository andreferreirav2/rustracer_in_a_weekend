use super::vec3::Vec3;
use Vec3 as Point3;

use super::ray::Ray;

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3
}

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self{
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Point3::new(viewport_width as f64, 0.0, 0.0);
        let vertical = Point3::new(0.0, viewport_height as f64, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            viewport_width,
            viewport_height,
            focal_length,

            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin, 
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
        }
    }
}