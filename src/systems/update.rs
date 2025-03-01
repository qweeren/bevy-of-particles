use bevy::prelude::*;
use rand::{prelude::*, thread_rng};
use crate::config::{GRID_HEIGHT, GRID_WIDTH};
use crate::grid::*;
use crate::materials::MaterialBehavior;
use crate::materials::Material;

pub fn update_grid(mut grid: ResMut<Grid>) {
    let mut new_grid = grid.clone();
    
    // Collect all non-empty cells
    let mut particles: Vec<(usize, usize)> = (0..GRID_HEIGHT)
        .flat_map(|y| (0..GRID_WIDTH).map(move |x| (x, y)))
        .filter(|&(x, y)| grid.get(x, y).material_type != Material::Empty as u8)
        .collect();

    // Randomize update order
    particles.shuffle(&mut thread_rng());

    // Update each particle using old state for reads, new state for writes
    for (x, y) in particles {
        if let Some(material) = get_material_at(x, y, &grid) {
            material.update(x, y, &mut new_grid, grid.as_ref());
        }
    }

    *grid = new_grid;
}

fn get_material_at(x: usize, y: usize, grid: &Grid) -> Option<Material> {
    let material = grid.get(x, y);
    if material.material_type == Material::Empty as u8 {
        return None;
    }
    
    Some(match material.material_type {
        1 => Material::Sand,
        2 => Material::Water,
        3 => Material::Concrete,
        4 => Material::Smoke,
        _ => Material::Empty,
    })
}
