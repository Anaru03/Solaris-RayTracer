use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let inv_x = 1.0 / ray.direction.x;
        let inv_y = 1.0 / ray.direction.y;
        let inv_z = 1.0 / ray.direction.z;

        let mut tx1 = (self.min.x - ray.origin.x) * inv_x;
        let mut tx2 = (self.max.x - ray.origin.x) * inv_x;

        if tx1 > tx2 {
            std::mem::swap(&mut tx1, &mut tx2);
        }

        let mut ty1 = (self.min.y - ray.origin.y) * inv_y;
        let mut ty2 = (self.max.y - ray.origin.y) * inv_y;

        if ty1 > ty2 {
            std::mem::swap(&mut ty1, &mut ty2);
        }

        let mut tz1 = (self.min.z - ray.origin.z) * inv_z;
        let mut tz2 = (self.max.z - ray.origin.z) * inv_z;

        if tz1 > tz2 {
            std::mem::swap(&mut tz1, &mut tz2);
        }

        let t_min = tx1.max(ty1).max(tz1);
        let t_max = tx2.min(ty2).min(tz2);

        if t_max < t_min || t_max < 0.001 {
            return None;
        }

        if t_min > 0.001 {
            Some(t_min)
        } else {
            Some(t_max)
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        let epsilon = 0.001;

        if (point.x - self.min.x).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }
}
