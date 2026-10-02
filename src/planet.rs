use crate::cube::Cube;
use crate::framebuffer::rgb;
use crate::material::Material;
use crate::vector::Vec3;

pub struct Planet {
    pub name: &'static str,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
    pub color: u32,
    pub material: Material,
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
        material: Material,
    ) -> Self {
        Self {
            name,
            radius,
            orbit_radius,
            orbit_speed,
            orbit_angle,
            color,
            material,
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
        let mut cubes = self.body_cubes();

        if self.name == "Saturn" {
            cubes.extend(self.saturn_ring_cubes());
        }

        cubes
    }

    fn body_cubes(&self) -> Vec<Cube> {
        let mut cubes = Vec::new();

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

                    if distance <= 1.18 {
                        let offset = Vec3::new(
                            x as f32 * block_size,
                            y as f32 * block_size,
                            z as f32 * block_size,
                        );

                        let center = self.position + offset;

                        cubes.push(Cube::new(
                            center - Vec3::new(half, half, half),
                            center + Vec3::new(half, half, half),
                        ));
                    }
                }
            }
        }

        cubes
    }

    fn saturn_ring_cubes(&self) -> Vec<Cube> {
        let mut cubes = Vec::new();

        let segments: usize = 56;

        let ring_radii = [self.radius * 1.45, self.radius * 1.90, self.radius * 2.35];

        let tilt = 0.22_f32;

        let cube_size = self.radius * 0.11;
        let half = cube_size * 0.5;

        for (band, ring_radius) in ring_radii.iter().enumerate() {
            let step = if band == 1 { 1 } else { 2 };

            for i in (0..segments).step_by(step) {
                let angle = i as f32 / segments as f32 * std::f32::consts::TAU;

                let local_x = ring_radius * angle.cos();

                let local_z = ring_radius * angle.sin();

                let local_y = local_z * tilt;

                let tilted_z = local_z * (1.0 - tilt * 0.20);

                let center = self.position + Vec3::new(local_x, local_y, tilted_z);

                cubes.push(Cube::new(
                    center - Vec3::new(half, half * 0.32, half),
                    center + Vec3::new(half, half * 0.32, half),
                ));
            }
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
            Material::mercury(),
        ),
        Planet::new(
            "Venus",
            0.42,
            3.0,
            1.20,
            0.7,
            rgb(210, 150, 65),
            Material::venus(),
        ),
        Planet::new(
            "Earth",
            0.46,
            4.2,
            1.00,
            1.4,
            rgb(35, 105, 210),
            Material::earth(),
        ),
        Planet::new(
            "Mars",
            0.35,
            5.4,
            0.80,
            2.0,
            rgb(195, 65, 40),
            Material::mars(),
        ),
        Planet::new(
            "Jupiter",
            0.90,
            7.2,
            0.43,
            2.6,
            rgb(195, 145, 105),
            Material::jupiter(),
        ),
        Planet::new(
            "Saturn",
            0.78,
            9.0,
            0.32,
            3.2,
            rgb(205, 180, 115),
            Material::saturn(),
        ),
        Planet::new(
            "Uranus",
            0.58,
            10.8,
            0.23,
            3.8,
            rgb(100, 195, 205),
            Material::uranus(),
        ),
        Planet::new(
            "Neptune",
            0.58,
            12.4,
            0.18,
            4.4,
            rgb(45, 75, 190),
            Material::neptune(),
        ),
    ]
}
