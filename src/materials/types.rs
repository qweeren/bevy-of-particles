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
