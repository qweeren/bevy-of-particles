use bevy::prelude::*;
use crate::config::GRID_HEIGHT;
use crate::config::GRID_WIDTH;
use crate::grid::*;
use crate::materials::MaterialBehavior;
use crate::materials::Material;

pub fn update_grid(mut grid: ResMut<Grid>) {
    let mut new_grid = grid.clone();
    update_grid_chunks(&mut new_grid, &grid);
    *grid = new_grid;
}

fn update_grid_chunks(new_grid: &mut Grid, old_grid: &Grid) {
    for chunk_y in (0..GRID_HEIGHT).step_by(3).rev() {
        let chunk_end = (chunk_y + 3).min(GRID_HEIGHT);
        update_chunk_rows(chunk_y, chunk_end, new_grid, old_grid);
    }
}

fn update_chunk_rows(start_y: usize, end_y: usize, new_grid: &mut Grid, old_grid: &Grid) {
    for y in start_y..end_y {
        for x in 0..GRID_WIDTH {
            if let Some(material) = get_material_at(x, y, old_grid) {
                material.update(x, y, new_grid);
            }
        }
    }
}

fn get_material_at(x: usize, y: usize, grid: &Grid) -> Option<Material> {
    let material = grid.get(x, y);
    if material == Material::Empty as u8 {
        return None;
    }
    
    Some(match material {
        1 => Material::Sand,
        2 => Material::Water,
        3 => Material::Concrete,
        4 => Material::Smoke,
        _ => Material::Empty,
    })
}