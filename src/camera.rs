use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            yaw: 0.8,
            pitch: 0.45,
            distance,
            fov: 60.0_f32.to_radians(),
        }
    }

    pub fn position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.distance * self.pitch.cos() * self.yaw.cos(),
            self.target.y + self.distance * self.pitch.sin(),
            self.target.z + self.distance * self.pitch.cos() * self.yaw.sin(),
        )
    }

    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;
        self.pitch = self.pitch.clamp(-1.3, 1.3);
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance + amount).clamp(1.2, 45.0);
    }

    pub fn get_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let eye = self.position();

        let forward = (self.target - eye).normalize();

        let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();

        let up = right.cross(&forward).normalize();

        let aspect = width as f32 / height as f32;

        let fov_scale = (self.fov / 2.0).tan();

        let px = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect * fov_scale;

        let py = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * fov_scale;

        let direction = (forward + right * px + up * py).normalize();

        Ray::new(eye, direction)
    }

    pub fn project(&self, point: Vec3, width: usize, height: usize) -> Option<(i32, i32)> {
        let eye = self.position();

        let forward = (self.target - eye).normalize();

        let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();

        let up = right.cross(&forward).normalize();

        let relative = point - eye;

        let camera_z = relative.dot(&forward);

        if camera_z <= 0.01 {
            return None;
        }

        let camera_x = relative.dot(&right);
        let camera_y = relative.dot(&up);

        let aspect = width as f32 / height as f32;
        let fov_scale = (self.fov / 2.0).tan();

        let ndc_x = camera_x / (camera_z * fov_scale * aspect);

        let ndc_y = camera_y / (camera_z * fov_scale);

        let screen_x = ((ndc_x + 1.0) * 0.5 * width as f32) as i32;

        let screen_y = ((1.0 - ndc_y) * 0.5 * height as f32) as i32;

        Some((screen_x, screen_y))
    }
}
