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

    pub fn sun() -> Self {
        Self::new(1.00, 0.05, 0.00, 0.00, 1.00)
    }

    pub fn mercury() -> Self {
        Self::new(0.70, 0.10, 0.00, 0.05, 1.00)
    }

    pub fn venus() -> Self {
        Self::new(0.82, 0.18, 0.00, 0.04, 1.00)
    }

    pub fn earth() -> Self {
        Self::new(0.85, 0.65, 0.00, 0.25, 1.00)
    }

    pub fn mars() -> Self {
        Self::new(0.75, 0.08, 0.00, 0.03, 1.00)
    }

    pub fn jupiter() -> Self {
        Self::new(0.88, 0.30, 0.00, 0.08, 1.00)
    }

    pub fn saturn() -> Self {
        Self::new(0.90, 0.35, 0.00, 0.12, 1.00)
    }

    pub fn uranus() -> Self {
        Self::new(0.82, 0.45, 0.08, 0.15, 1.02)
    }

    pub fn neptune() -> Self {
        Self::new(0.80, 0.50, 0.05, 0.18, 1.02)
    }

    pub fn ice() -> Self {
        Self::new(0.30, 0.85, 0.70, 0.15, 1.31)
    }
}
