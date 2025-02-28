use crate::materials::Material;

pub static MATERIAL_NAMES: [&str; 5] = [
    "Empty",
    "Sand",
    "Water",
    "Concrete",
    "Smoke",
];

pub fn material_from_id(id: u8) -> Material {
    match id {
        0 => Material::Empty,
        1 => Material::Sand,
        2 => Material::Water,
        3 => Material::Concrete,
        4 => Material::Smoke,
        _ => Material::Empty,
    }
}