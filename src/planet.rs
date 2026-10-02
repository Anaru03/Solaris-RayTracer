use crate::cube::Cube;
use crate::framebuffer::rgb;
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

    pub fn moon_position(&self, angle: f32) -> Vec3 {
        self.position + Vec3::new(0.75 * angle.cos(), 0.08, 0.75 * angle.sin())
    }

    pub fn voxel_cubes(&self) -> Vec<Cube> {
        create_voxel_sphere(self.position, self.radius)
    }
}

pub fn create_voxel_sphere(center: Vec3, radius: f32) -> Vec<Cube> {
    let mut cubes = Vec::new();

    let resolution = 3;
    let size = radius * 2.0 / resolution as f32;

    for x in -resolution..=resolution {
        for y in -resolution..=resolution {
            for z in -resolution..=resolution {
                let offset = Vec3::new(x as f32 * size, y as f32 * size, z as f32 * size);

                if offset.length() <= radius {
                    let cube_center = center + offset;

                    let half = size * 0.55;

                    cubes.push(Cube::new(
                        cube_center - Vec3::new(half, half, half),
                        cube_center + Vec3::new(half, half, half),
                    ));
                }
            }
        }
    }

    cubes
}

pub fn create_planets() -> Vec<Planet> {
    vec![
        Planet::new("Mercury", 0.30, 2.0, 1.60, 0.0, rgb(145, 140, 130)),
        Planet::new("Venus", 0.42, 3.0, 1.20, 0.7, rgb(205, 145, 65)),
        Planet::new("Earth", 0.46, 4.2, 1.00, 1.4, rgb(35, 105, 210)),
        Planet::new("Mars", 0.35, 5.4, 0.80, 2.0, rgb(190, 65, 40)),
        Planet::new("Jupiter", 0.90, 7.2, 0.43, 2.6, rgb(195, 145, 105)),
        Planet::new("Saturn", 0.78, 9.0, 0.32, 3.2, rgb(205, 180, 115)),
        Planet::new("Uranus", 0.58, 10.8, 0.23, 3.8, rgb(100, 195, 205)),
        Planet::new("Neptune", 0.58, 12.4, 0.18, 4.4, rgb(45, 75, 190)),
    ]
}
