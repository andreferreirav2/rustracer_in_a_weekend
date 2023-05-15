use super::vec3::Vec3;
use Vec3 as Color;
use Vec3 as Point3;

use super::ray::Ray;
use super::camera::Camera;
use super::shapes::{Sphere, HitResult, Hittable, HitPoint, HittableList};

const WHITE: Color = Color{i: 1.0, j: 1.0, k: 1.0};
const LIGHT_BLUE: Color = Color{i: 0.5, j: 0.7, k: 1.0};

pub struct World {
    width: u32,
    height: u32,
    camera: Camera,
    elements: HittableList
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {width, height,
            camera: Camera::new(2.0, (width as f64) / (height as f64), 1.0),
            elements: HittableList::new(vec![
                Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
                Box::new(Sphere::new(Point3::new(2.0, -1.5, -1.0), 2.0)),
            ])}
    }

    pub fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as f64;
            let y = (i / self.width as usize) as f64;

            let u = x / (self.width as f64);
            let v = 1.0 - y / (self.height as f64);

            let color = self.cast_ray(self.camera.ray_at(u, v));
            let rgba = color.as_rgba();

            pixel.copy_from_slice(&rgba);
        }
    }
    
    fn cast_ray(&self, ray: Ray) -> Color {
        match self.elements.hit(&ray, 0.0, 10.0) {
            HitResult::NoHit => {
                let unit_direction = ray.dir.normalized();
                let t = 0.5*(unit_direction.j + 1.0);
                
                WHITE * (1.0-t) + LIGHT_BLUE * t
            }
            HitResult::Hit(HitPoint{point: _, normal, t: _, face: _}) => {
                (normal + WHITE) * 0.5
            }
        }
    }
}
