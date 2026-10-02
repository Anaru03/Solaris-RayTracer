use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();

        let t1 = (-b - root) / (2.0 * a);
        let t2 = (-b + root) / (2.0 * a);

        if t1 > 0.001 {
            Some(t1)
        } else if t2 > 0.001 {
            Some(t2)
        } else {
            None
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}
