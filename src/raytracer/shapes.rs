use super::vec3::Vec3;
use Vec3 as Point3;

use super::ray::Ray;

pub enum HitFace {
    Front,
    Back
}

pub struct HitPoint {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub face: HitFace
}

pub enum HitResult {
    NoHit,
    Hit(HitPoint)
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult;
}

////////////////////////////


pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self{
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let offset: Vec3 = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(&offset, &ray.dir);
        let c = offset.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
    
        if discriminant < 0.0 {
            return HitResult::NoHit;
        }
        let sqrtd = discriminant.sqrt();

        let mut dist = (-half_b - sqrtd) / a;
        if dist < t_min || t_max < dist {
            dist = (-half_b + sqrtd) / a;
            if dist < t_min || t_max < dist {
                return HitResult::NoHit;
            }
        }

        let point = ray.at(dist);
        let outward_normal = (point - self.center) / self.radius;
        let (face, normal) =  if Vec3::dot(&ray.dir, &outward_normal) < 0.0 {
            (HitFace::Front, outward_normal)
        }
        else { 
            (HitFace::Back, -outward_normal)
        };

        HitResult::Hit(HitPoint{point, normal, t: dist, face})
    }

}