use super::vec3::Vec3;
use Vec3 as Color;
use Vec3 as Point3;

use super::ray::Ray;
use super::camera::Camera;

const WHITE: Color = Color{i: 1.0, j: 1.0, k: 1.0};
const LIGHT_BLUE: Color = Color{i: 0.5, j: 0.7, k: 1.0};
const RED: Color = Color{i: 1.0, j: 0.0, k: 0.0};

pub struct World {
    width: u32,
    height: u32,
    camera: Camera
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {width, height, camera: Camera::new(2.0, (width as f64) / (height as f64), 1.0)}
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

            let color = ray_color(self.camera.ray_at(u, v));
            let rgba = color.as_rgba();

            pixel.copy_from_slice(&rgba);
        }
    }
}

fn ray_color(ray: Ray) -> Color {
    match cast_ray_into_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        RayCastResult::NoHit => {
            let unit_direction = ray.dir.normalized();
            let t = 0.5*(unit_direction.j + 1.0);
            
            WHITE * (1.0-t) + LIGHT_BLUE * t
        }
        RayCastResult::Hit(t) => {
            let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            (normal + WHITE) * 0.5
        }
    }
}

enum RayCastResult {
    Hit(f64),
    NoHit
}

fn cast_ray_into_sphere(center: Point3, radius: f64, ray: &Ray) -> RayCastResult {
    let offset: Vec3 = ray.origin - center;
    let a = Vec3::dot(&ray.dir, &ray.dir);
    let b = 2.0 * Vec3::dot(&offset, &ray.dir);
    let c = Vec3::dot(&offset, &offset) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        RayCastResult::NoHit
    } else {
        RayCastResult::Hit((-b - discriminant.sqrt() ) / (2.0*a))
    }
}
