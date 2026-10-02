#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: f32,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        albedo: f32,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            albedo,
            specular,
            transparency,
            reflectivity,
            refractive_index,
        }
    }

    pub fn mercury() -> Self {
        Self::new(0.70, 0.10, 0.00, 0.05, 1.0)
    }

    pub fn venus() -> Self {
        Self::new(0.80, 0.20, 0.00, 0.05, 1.0)
    }

    pub fn earth() -> Self {
        Self::new(0.85, 0.55, 0.00, 0.20, 1.0)
    }

    pub fn mars() -> Self {
        Self::new(0.75, 0.08, 0.00, 0.03, 1.0)
    }

    pub fn jupiter() -> Self {
        Self::new(0.80, 0.30, 0.00, 0.08, 1.0)
    }

    pub fn saturn() -> Self {
        Self::new(0.82, 0.35, 0.00, 0.12, 1.0)
    }

    pub fn ice() -> Self {
        Self::new(0.25, 0.80, 0.75, 0.15, 1.31)
    }
}
