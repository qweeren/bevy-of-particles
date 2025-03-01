use super::types::Material;

pub struct MaterialProperties {
    pub color: (u8, u8, u8),
    pub density: f32,
    pub viscosity: f32,
}

impl Material {
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::Empty => MaterialProperties {
                color: (0, 0, 0),
                density: 0.0,
                viscosity: 1.0,
            },
            Material::Sand => MaterialProperties {
                color: (194, 178, 128),
                density: 1.6,
                viscosity: 0.9, // Sand barely flows
            },
            Material::Water => MaterialProperties {
                color: (0, 119, 190),
                density: 1.0,
                viscosity: 0.1, // Reduced significantly to make water much more fluid
            },
            Material::Concrete => MaterialProperties {
                color: (128, 128, 128),
                density: 2.4,
                viscosity: 1.0, // Concrete doesn't flow
            },
            Material::Smoke => MaterialProperties {
                color: (200, 200, 200),
                density: 0.1,
                viscosity: 0.1, // Smoke spreads very easily
            },
        }
    }
}
