use super::vec3::Vec3;
use Vec3 as Color;
use Vec3 as Point3;

use super::ray::Ray;

pub struct World {
    width: u32,
    height: u32
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {width, height}
    }

    pub fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as f64;
            let y = (i / self.width as usize) as f64;

            let color = Color{i: x / ((self.width-1) as f64), j: y / ((self.height-1) as f64), k: 0.25};
            let rgba = color.as_rgba();

            pixel.copy_from_slice(&rgba);
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.dir.normalized();
    let t = 0.5*(unit_direction.j + 1.0);
    let white = Color{i: 1.0, j: 1.0, k: 1.0};
    let light_blue = Color{i: 0.5, j: 0.7, k: 1.0};
    
    white * (1.0-t) + light_blue * t
}
