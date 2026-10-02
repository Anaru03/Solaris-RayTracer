use crate::framebuffer::rgb;
use crate::sphere::Sphere;
use crate::vector::Vec3;

pub struct Planet {
    pub name: &'static str,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
    pub color: u32,
    pub position: Vec3,
}

impl Planet {
    pub fn new(
        name: &'static str,
        radius: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        orbit_angle: f32,
        color: u32,
    ) -> Self {
        Self {
            name,
            radius,
            orbit_radius,
            orbit_speed,
            orbit_angle,
            color,
            position: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.orbit_angle += self.orbit_speed * dt;

        self.position = Vec3::new(
            self.orbit_radius * self.orbit_angle.cos(),
            0.0,
            self.orbit_radius * self.orbit_angle.sin(),
        );
    }

    pub fn sphere(&self) -> Sphere {
        Sphere::new(self.position, self.radius)
    }
}

pub fn create_planets() -> Vec<Planet> {
    vec![
        Planet::new("Mercury", 0.25, 2.0, 1.60, 0.0, rgb(150, 145, 135)),
        Planet::new("Venus", 0.38, 3.0, 1.20, 0.7, rgb(210, 155, 75)),
        Planet::new("Earth", 0.42, 4.2, 1.00, 1.4, rgb(35, 105, 210)),
        Planet::new("Mars", 0.32, 5.4, 0.80, 2.0, rgb(190, 65, 40)),
        Planet::new("Jupiter", 0.85, 7.2, 0.43, 2.6, rgb(195, 145, 105)),
        Planet::new("Saturn", 0.72, 9.0, 0.32, 3.2, rgb(205, 180, 115)),
        Planet::new("Uranus", 0.55, 10.8, 0.23, 3.8, rgb(100, 195, 205)),
        Planet::new("Neptune", 0.54, 12.4, 0.18, 4.4, rgb(45, 75, 190)),
    ]
}
