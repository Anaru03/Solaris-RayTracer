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
    pub secondary_color: u32,
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
        secondary_color: u32,
    ) -> Self {
        Self {
            name,
            radius,
            orbit_radius,
            orbit_speed,
            orbit_angle,
            color,
            secondary_color,
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

    pub fn voxel_cubes(&self) -> Vec<(Cube, u32)> {
        let mut cubes = Vec::new();

        let resolution = 4;
        let size = self.radius * 2.0 / resolution as f32;
        let half = size * 0.52;

        for x in -resolution..=resolution {
            for y in -resolution..=resolution {
                for z in -resolution..=resolution {
                    let offset = Vec3::new(x as f32 * size, y as f32 * size, z as f32 * size);

                    let distance = offset.length();

                    if distance <= self.radius && distance >= self.radius * 0.50 {
                        let center = self.position + offset;

                        cubes.push((
                            Cube::new(
                                center - Vec3::new(half, half, half),
                                center + Vec3::new(half, half, half),
                            ),
                            self.voxel_color(x, y, z),
                        ));
                    }
                }
            }
        }

        cubes
    }

    fn voxel_color(&self, x: i32, y: i32, z: i32) -> u32 {
        match self.name {
            "Mercury" => match (x * 3 + y * 5 + z * 7).abs() % 8 {
                0..=2 => rgb(90, 88, 84),
                3..=4 => rgb(125, 120, 112),
                _ => rgb(160, 155, 145),
            },

            "Venus" => match (x * 2 + y * 3 + z * 5).abs() % 7 {
                0..=2 => rgb(225, 175, 85),
                3..=4 => rgb(185, 120, 50),
                _ => rgb(240, 205, 115),
            },

            "Earth" => {
                if y >= 3 || y <= -3 {
                    rgb(235, 240, 245)
                } else {
                    let pattern = (x * 3 + z * 5 + y * 2).abs() % 11;

                    if pattern <= 3 {
                        rgb(45, 145, 65)
                    } else if pattern == 4 {
                        rgb(80, 175, 85)
                    } else {
                        rgb(25, 90, 195)
                    }
                }
            }

            "Mars" => match (x * 5 + y * 3 + z * 7).abs() % 9 {
                0..=2 => rgb(105, 40, 28),
                3..=4 => rgb(155, 55, 35),
                _ => rgb(205, 75, 45),
            },

            "Jupiter" => match y.rem_euclid(6) {
                0 => rgb(235, 215, 180),
                1 => rgb(205, 165, 125),
                2 => rgb(150, 90, 60),
                3 => rgb(225, 195, 155),
                4 => rgb(180, 120, 80),
                _ => rgb(240, 220, 190),
            },

            "Saturn" => match y.rem_euclid(4) {
                0 => rgb(235, 215, 155),
                1 => rgb(205, 180, 120),
                2 => rgb(245, 225, 170),
                _ => rgb(190, 165, 105),
            },

            "Uranus" => {
                if (x + y * 2 + z).abs() % 5 <= 1 {
                    rgb(155, 225, 230)
                } else {
                    rgb(90, 185, 205)
                }
            }

            "Neptune" => {
                if (x + y * 3 + z * 2).abs() % 7 <= 1 {
                    rgb(75, 120, 235)
                } else {
                    rgb(35, 65, 180)
                }
            }

            _ => self.color,
        }
    }
}

pub fn create_planets() -> Vec<Planet> {
    vec![
        Planet::new(
            "Mercury",
            0.30,
            2.0,
            1.60,
            0.0,
            rgb(145, 140, 130),
            rgb(95, 92, 88),
        ),
        Planet::new(
            "Venus",
            0.42,
            3.0,
            1.20,
            0.7,
            rgb(210, 150, 65),
            rgb(235, 190, 95),
        ),
        Planet::new(
            "Earth",
            0.46,
            4.2,
            1.00,
            1.4,
            rgb(35, 105, 210),
            rgb(45, 145, 70),
        ),
        Planet::new(
            "Mars",
            0.35,
            5.4,
            0.80,
            2.0,
            rgb(195, 65, 40),
            rgb(110, 40, 28),
        ),
        Planet::new(
            "Jupiter",
            0.90,
            7.2,
            0.43,
            2.6,
            rgb(195, 145, 105),
            rgb(230, 195, 150),
        ),
        Planet::new(
            "Saturn",
            0.78,
            9.0,
            0.32,
            3.2,
            rgb(205, 180, 115),
            rgb(235, 215, 155),
        ),
        Planet::new(
            "Uranus",
            0.58,
            10.8,
            0.23,
            3.8,
            rgb(100, 195, 205),
            rgb(145, 220, 225),
        ),
        Planet::new(
            "Neptune",
            0.58,
            12.4,
            0.18,
            4.4,
            rgb(45, 75, 190),
            rgb(55, 110, 225),
        ),
    ]
}
