use crate::grid::Grid;
use crate::materials::Material;
use crate::config;
use rand::{prelude::*, rng};

pub trait MaterialBehavior {
    fn update(&self, x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid);
}

impl MaterialBehavior for Material {
    fn update(&self, x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) {
        // Skip if already updated in this frame
        if new_grid.get(x, y) != old_grid.get(x, y) {
            return;
        }

        match self {
            Material::Sand => {
                if !try_move_density_based(x, y, new_grid, old_grid) {
                    if !fall(x, y, new_grid, old_grid) {
                        new_grid.set(x, y, old_grid.get(x, y));
                    }
                }
            }
            Material::Water => {
                if !try_move_density_based(x, y, new_grid, old_grid) {
                    if !fall(x, y, new_grid, old_grid) {
                        flow(x, y, new_grid, old_grid);
                    }
                }
            }
            Material::Smoke => {
                if !try_move_density_based(x, y, new_grid, old_grid) {
                    if !rise(x, y, new_grid, old_grid) {
                        flow(x, y, new_grid, old_grid);
                    }
                }
            }
            Material::Concrete => {
                new_grid.set(x, y, Material::Concrete as u8);
            }
            Material::Empty => {}
        }
    }
}

fn try_move_density_based(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) -> bool {
    let current_material = Material::from_id(old_grid.get(x, y));
    let current_density = current_material.properties().density;

    let mut check_and_swap = |x2: usize, y2: usize| -> bool {
        let other_material = Material::from_id(old_grid.get(x2, y2));
        let other_density = other_material.properties().density;
        
        if other_material != Material::Empty && current_density > other_density {
            new_grid.swap(x, y, x2, y2);
            true
        } else {
            false
        }
    };

    // Check below first
    if y < config::GRID_HEIGHT - 1 && check_and_swap(x, y + 1) {
        return true;
    }

    // Then check diagonally below
    if y < config::GRID_HEIGHT - 1 {
        let left = x > 0;
        let right = x < config::GRID_WIDTH - 1;
        
        match (left, right) {
            (true, true) => {
                if rng().random::<bool>() {
                    if check_and_swap(x - 1, y + 1) { return true; }
                    if check_and_swap(x + 1, y + 1) { return true; }
                } else {
                    if check_and_swap(x + 1, y + 1) { return true; }
                    if check_and_swap(x - 1, y + 1) { return true; }
                }
            }
            (true, false) => if check_and_swap(x - 1, y + 1) { return true; }
            (false, true) => if check_and_swap(x + 1, y + 1) { return true; }
            (false, false) => {}
        }
    }

    false
}

fn rise(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) -> bool {
    if y > 0 && old_grid.get(x, y - 1) == Material::Empty as u8 {
        new_grid.move_to(x, y, x, y - 1);
        return true;
    }
    if y > 0 {
        let left = x > 0 && old_grid.get(x - 1, y - 1) == Material::Empty as u8;
        let right = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y - 1) == Material::Empty as u8;
        
        match (left, right) {
            (true, true) => {
                if rng().random::<bool>() {
                    new_grid.move_to(x, y, x - 1, y - 1)
                } else {
                    new_grid.move_to(x, y, x + 1, y - 1)
                }
                true
            }
            (true, false) => {
                new_grid.move_to(x, y, x - 1, y - 1);
                true
            }
            (false, true) => {
                new_grid.move_to(x, y, x + 1, y - 1);
                true
            }
            (false, false) => false
        }
    } else {
        false
    }
}

fn fall(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) -> bool {
    if y < config::GRID_HEIGHT - 1 && old_grid.get(x, y + 1) == Material::Empty as u8 {
        new_grid.move_to(x, y, x, y + 1);
        return true;
    }

    if y < config::GRID_HEIGHT - 1 {
        let left = x > 0 && old_grid.get(x - 1, y + 1) == Material::Empty as u8;
        let right = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y + 1) == Material::Empty as u8;
        
        match (left, right) {
            (true, true) => {
                if rng().random::<bool>() {
                    new_grid.move_to(x, y, x - 1, y + 1)
                } else {
                    new_grid.move_to(x, y, x + 1, y + 1)
                }
                true
            }
            (true, false) => {
                new_grid.move_to(x, y, x - 1, y + 1);
                true
            }
            (false, true) => {
                new_grid.move_to(x, y, x + 1, y + 1);
                true
            }
            (false, false) => false
        }
    } else {
        false
    }
}

fn flow(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) {
    let mut rng = rng();
    let left = x > 0 && old_grid.get(x - 1, y) == Material::Empty as u8;
    let right = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y) == Material::Empty as u8;

    match (left, right) {
        (true, true) => {
            if rng.random::<bool>() {
                new_grid.move_to(x, y, x + 1, y);
            } else {
                new_grid.move_to(x, y, x - 1, y);
            }
        }
        (true, false) => new_grid.move_to(x, y, x - 1, y),
        (false, true) => new_grid.move_to(x, y, x + 1, y),
        (false, false) => new_grid.set(x, y, old_grid.get(x, y)),
    }
}

