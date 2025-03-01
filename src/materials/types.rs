#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Empty = 0,
    Sand = 1,
    Water = 2,
    Concrete = 3,
    Smoke = 4,
}

impl Material {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Material::Empty,
            1 => Material::Sand,
            2 => Material::Water,
            3 => Material::Concrete,
            4 => Material::Smoke,
            _ => Material::Empty,
        }
    }
}

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ParticleFlags: u8 {
        const MOVABLE     = 0b0000_0001;
        const FLOWS       = 0b0000_0010;
        const RISES       = 0b0000_0100;
        const DISPERSES   = 0b0000_1000;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub material_type: u8,    // 8 bits for material type
    pub flags: ParticleFlags, // 8 bits for behavior flags
    pub properties: u16,      // 16 bits for compressed properties
}

impl Particle {
    pub fn new(material_type: Material) -> Self {
        let (flags, properties) = match material_type {
            Material::Empty => (
                ParticleFlags::empty(),
                0,
            ),
            Material::Sand => (
                ParticleFlags::MOVABLE,
                Self::pack_properties(194, 178, 128, 16, 9),
            ),
            Material::Water => (
                ParticleFlags::MOVABLE | ParticleFlags::FLOWS,
                Self::pack_properties(0, 119, 190, 10, 1),
            ),
            Material::Smoke => (
                ParticleFlags::MOVABLE | ParticleFlags::FLOWS | ParticleFlags::RISES | ParticleFlags::DISPERSES,
                Self::pack_properties(200, 200, 200, 1, 1),
            ),
            Material::Concrete => (
                ParticleFlags::empty(),
                Self::pack_properties(128, 128, 128, 24, 10),
            ),
        };

        Self {
            material_type: material_type as u8,
            flags,
            properties,
        }
    }

    // Pack density (0-31) and viscosity (0-15) into the upper 8 bits
    // Pack RGB colors into the lower 8 bits using 3-3-2 format
    fn pack_properties(r: u8, g: u8, b: u8, density: u8, viscosity: u8) -> u16 {
        let density = (density & 0b1_1111) as u16;      // 5 bits for density
        let viscosity = (viscosity & 0b1111) as u16;    // 4 bits for viscosity
        let r = ((r >> 5) & 0b111) as u16;             // 3 bits for red
        let g = ((g >> 5) & 0b111) as u16;             // 3 bits for green
        let b = ((b >> 6) & 0b11) as u16;              // 2 bits for blue

        (density << 11) | (viscosity << 7) | (r << 4) | (g << 1) | b
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        let r = ((self.properties >> 4) & 0b111) << 5;
        let g = ((self.properties >> 1) & 0b111) << 5;
        let b = (self.properties & 0b11) << 6;
        (r as u8, g as u8, b as u8)
    }

    pub fn get_density(&self) -> f32 {
        ((self.properties >> 11) & 0b1_1111) as f32 / 10.0
    }

    pub fn get_viscosity(&self) -> f32 {
        ((self.properties >> 7) & 0b1111) as f32 / 10.0
    }
}
