use super::types::Material;

#[derive(Debug, Clone, Copy)]
pub struct MaterialProperties {
    pub color: (u8, u8, u8),
    pub density: u8,    // Changed to u8 (0-31)
    pub viscosity: u8,  // Changed to u8 (0-15)
}

impl Material {
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::Empty => MaterialProperties {
                color: (0, 0, 0),
                density: 0,
                viscosity: 0,
            },
            Material::Sand => MaterialProperties {
                color: (194, 178, 128),
                density: 16,  // 1.6 * 10
                viscosity: 9, // 0.9 * 10
            },
            Material::Water => MaterialProperties {
                color: (0, 119, 190),
                density: 10,  // 1.0 * 10
                viscosity: 1, // 0.1 * 10
            },
            Material::Concrete => MaterialProperties {
                color: (128, 128, 128),
                density: 24,  // 2.4 * 10
                viscosity: 10,// 1.0 * 10
            },
            Material::Smoke => MaterialProperties {
                color: (200, 200, 200),
                density: 1,   // 0.1 * 10
                viscosity: 1, // 0.1 * 10
            },
            Material::Fire => MaterialProperties {
                color: (255, 100, 0),  // More orange base color
                density: 1,   // Very light
                viscosity: 2, // Slightly more viscous for better shape
            },
        }
    }
}
