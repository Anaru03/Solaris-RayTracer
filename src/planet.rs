use crate::cube::Cube;
use crate::framebuffer::rgb;
use crate::texture::Texture;
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

    pub fn voxel_cubes(&self, texture: &Texture) -> Vec<(Cube, u32)> {
        let mut cubes = Vec::new();

        // 5 bloques de diámetro aproximado.
        // Mucho más redondo que antes sin ser exageradamente pesado.
        let grid_radius: i32 = 2;

        let block_size = self.radius / 2.25;

        let half = block_size * 0.51;

        for x in -grid_radius..=grid_radius {
            for y in -grid_radius..=grid_radius {
                for z in -grid_radius..=grid_radius {
                    let nx = x as f32 / grid_radius as f32;

                    let ny = y as f32 / grid_radius as f32;

                    let nz = z as f32 / grid_radius as f32;

                    let distance = (nx * nx + ny * ny + nz * nz).sqrt();

                    // Esfera voxel.
                    if distance <= 1.18 {
                        let offset = Vec3::new(
                            x as f32 * block_size,
                            y as f32 * block_size,
                            z as f32 * block_size,
                        );

                        let center = self.position + offset;

                        let color = self.planet_color(x, y, z, texture);

                        cubes.push((
                            Cube::new(
                                center - Vec3::new(half, half, half),
                                center + Vec3::new(half, half, half),
                            ),
                            color,
                        ));
                    }
                }
            }
        }

        cubes
    }

    fn planet_color(&self, x: i32, y: i32, z: i32, texture: &Texture) -> u32 {
        // La textura real sigue participando.
        // La usamos para generar variación de luminosidad
        // en vez de pegar pedazos arbitrarios de la fotografía.
        let u = ((x + 2) as f32 / 5.0).clamp(0.0, 0.99);

        let v = ((y + 2) as f32 / 5.0).clamp(0.0, 0.99);

        let sample = texture.sample(u, v);

        let sr = ((sample >> 16) & 255) as f32;

        let sg = ((sample >> 8) & 255) as f32;

        let sb = (sample & 255) as f32;

        let texture_luminance = (sr + sg + sb) / (255.0 * 3.0);

        let variation = 0.78 + texture_luminance * 0.35;

        let base = match self.name {
            "Mercury" => {
                let p = (x * 7 + y * 3 + z * 5).abs() % 5;

                if p == 0 {
                    rgb(90, 88, 84)
                } else if p <= 2 {
                    rgb(135, 132, 125)
                } else {
                    rgb(170, 168, 160)
                }
            }

            "Venus" => {
                let p = (x * 5 + y * 3 + z).abs() % 5;

                if p <= 1 {
                    rgb(185, 115, 45)
                } else if p == 2 {
                    rgb(235, 175, 70)
                } else {
                    rgb(215, 145, 55)
                }
            }

            "Earth" => {
                // Polos
                if y >= 2 || y <= -2 {
                    rgb(235, 240, 245)
                } else {
                    // Continentes reconocibles.
                    let continent = (x * 7 + z * 5 + y * 3).abs() % 10;

                    if continent <= 3 {
                        if continent == 0 {
                            rgb(125, 150, 65)
                        } else {
                            rgb(40, 135, 65)
                        }
                    } else {
                        if (x + z).abs() % 3 == 0 {
                            rgb(35, 115, 205)
                        } else {
                            rgb(20, 75, 175)
                        }
                    }
                }
            }

            "Mars" => {
                let p = (x * 5 + y * 7 + z * 3).abs() % 6;

                if p == 0 {
                    rgb(100, 35, 25)
                } else if p <= 2 {
                    rgb(165, 55, 35)
                } else {
                    rgb(205, 75, 45)
                }
            }

            "Jupiter" => {
                // Franjas horizontales.
                match y.rem_euclid(5) {
                    0 => rgb(235, 215, 180),
                    1 => rgb(185, 125, 85),
                    2 => rgb(225, 185, 145),
                    3 => rgb(145, 90, 60),
                    _ => rgb(240, 220, 190),
                }
            }

            "Saturn" => match y.rem_euclid(4) {
                0 => rgb(235, 215, 155),
                1 => rgb(200, 175, 110),
                2 => rgb(245, 225, 175),
                _ => rgb(215, 195, 135),
            },

            "Uranus" => {
                if y.rem_euclid(3) == 0 {
                    rgb(145, 220, 225)
                } else {
                    rgb(85, 185, 205)
                }
            }

            "Neptune" => {
                let p = (x + y * 3 + z * 2).abs() % 6;

                if p <= 1 {
                    rgb(75, 120, 230)
                } else {
                    rgb(35, 65, 180)
                }
            }

            _ => self.color,
        };

        scale_color(base, variation)
    }
}

fn scale_color(color: u32, factor: f32) -> u32 {
    let r = ((color >> 16) & 255) as f32;

    let g = ((color >> 8) & 255) as f32;

    let b = (color & 255) as f32;

    rgb(
        (r * factor).clamp(0.0, 255.0) as u32,
        (g * factor).clamp(0.0, 255.0) as u32,
        (b * factor).clamp(0.0, 255.0) as u32,
    )
}

pub fn create_planets() -> Vec<Planet> {
    vec![
        Planet::new("Mercury", 0.30, 2.0, 1.60, 0.0, rgb(145, 140, 130)),
        Planet::new("Venus", 0.42, 3.0, 1.20, 0.7, rgb(210, 150, 65)),
        Planet::new("Earth", 0.46, 4.2, 1.00, 1.4, rgb(35, 105, 210)),
        Planet::new("Mars", 0.35, 5.4, 0.80, 2.0, rgb(195, 65, 40)),
        Planet::new("Jupiter", 0.90, 7.2, 0.43, 2.6, rgb(195, 145, 105)),
        Planet::new("Saturn", 0.78, 9.0, 0.32, 3.2, rgb(205, 180, 115)),
        Planet::new("Uranus", 0.58, 10.8, 0.23, 3.8, rgb(100, 195, 205)),
        Planet::new("Neptune", 0.58, 12.4, 0.18, 4.4, rgb(45, 75, 190)),
    ]
}
