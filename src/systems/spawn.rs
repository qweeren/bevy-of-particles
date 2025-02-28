suse bevy::prelude::*;
use crate::grid::Grid;
use crate::materials::Material;

pub fn spawn_test_materials(mut grid: ResMut<Grid>) {
    grid.set(50, 10, Material::Sand as u8);
    grid.set(40, 20, Material::Water as u8);
    grid.set(60, 30, Material::Concrete as u8);
    grid.set(50, 40, Material::Smoke as u8);
}