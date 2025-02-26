use bevy::prelude::*;
use crate::grid::{self, Grid};
use crate::materials::{Material, MaterialBehavior};

pub(crate) fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2d::default());
}

pub fn update_grid(mut grid: ResMut<Grid>) {
    let mut new_grid = grid.clone();
    for y in 0..grid::GRID_HEIGHT {
        for x in 0..grid::GRID_WIDTH {
            let material = grid.get(x, y);
            if material != Material::Empty as u8 {
                let material_enum = match material {
                    1 => Material::Sand,
                    2 => Material::Water,
                    3 => Material::Concrete,
                    4 => Material::Smoke,
                    _ => Material::Empty,
                };
                material_enum.update(x, y, &mut new_grid);
            }
        }
    }
    *grid = new_grid;
}