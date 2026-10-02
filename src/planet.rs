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

        let resolution = 3;
        let size = self.radius * 2.0 / resolution as f32;
        let half = size * 0.55;

        for x in -resolution..=resolution {
            for y in -resolution..=resolution {
                for z in -resolution..=resolution {
                    let offset = Vec3::new(x as f32 * size, y as f32 * size, z as f32 * size);

                    if offset.length() <= self.radius {
                        let cube_center = self.position + offset;

                        let color = self.voxel_color(x, y, z);

                        cubes.push((
                            Cube::new(
                                cube_center - Vec3::new(half, half, half),
                                cube_center + Vec3::new(half, half, half),
                            ),
                            color,
                        ));
                    }
                }
            }
        }

        cubes
    }

    fn voxel_color(&self, x: i32, y: i32, z: i32) -> u32 {
        match self.name {
            "Mercury" => {
                if (x + y + z).abs() % 3 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            "Venus" => {
                if (x + z).abs() % 2 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            "Earth" => {
                if y >= 2 || y <= -2 {
                    rgb(225, 235, 240)
                } else if (x + z * 2 + y).abs() % 4 == 0 || (x - z).abs() % 5 == 0 {
                    rgb(45, 145, 70)
                } else {
                    self.color
                }
            }

            "Mars" => {
                if (x * 2 + y + z).abs() % 4 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            "Jupiter" => {
                if y % 3 == 0 {
                    rgb(225, 190, 145)
                } else if y % 2 == 0 {
                    rgb(165, 105, 70)
                } else {
                    self.color
                }
            }

            "Saturn" => {
                if y % 2 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            "Uranus" => {
                if y.abs() % 2 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            "Neptune" => {
                if (x + y + z).abs() % 4 == 0 {
                    self.secondary_color
                } else {
                    self.color
                }
            }

            _ => self.color,
        }
    }

    pub fn orbit_cubes(&self) -> Vec<Cube> {
        let mut cubes = Vec::new();

        let segments = 72;
        let size = 0.035;

        for i in 0..segments {
            let angle = i as f32 / segments as f32 * std::f32::consts::TAU;

            let center = Vec3::new(
                self.orbit_radius * angle.cos(),
                -0.08,
                self.orbit_radius * angle.sin(),
            );

            cubes.push(Cube::new(
                center - Vec3::new(size, size, size),
                center + Vec3::new(size, size, size),
            ));
        }

        cubes
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
