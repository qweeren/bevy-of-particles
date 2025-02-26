use bevy::color::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Empty = 0,    // No material
    Sand = 1,     // Powder type
    Water = 2,    // Liquid type
    Concrete = 3, // Solid type
    Smoke = 4,    // Gas type
}

impl Material {
    pub fn color(&self) -> Color {
        match self {
            Material::Empty => Color::BLACK,
            Material::Sand => Color::srgb(0.76, 0.70, 0.50),    // Sandy beige
            Material::Water => Color::srgb(0.0, 0.5, 1.0),     // Light blue
            Material::Concrete => Color::srgb(0.5, 0.5, 0.5),  // Gray
            Material::Smoke => Color::srgb(0.8, 0.8, 0.8),     // Light gray
        }
    }
}

pub const MATERIAL_NAMES: [&str; 5] = [
    "Erase",      // MATERIAL_EMPTY = 0
    "Sand",       // MATERIAL_SAND = 1
    "Water",      // MATERIAL_WATER = 2
    "Concrete",   // MATERIAL_CONCRETE = 3
    "Smoke",      // MATERIAL_SMOKE = 4
];