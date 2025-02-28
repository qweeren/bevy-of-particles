use crate::grid::{self, Grid};
use crate::materials::Material;
use crate::config;


pub trait MaterialBehavior {
    fn update(&self, x: usize, y: usize, grid: &mut Grid);
}

impl MaterialBehavior for Material {
    fn update(&self, x: usize, y: usize, grid: &mut Grid) {
        match self {
            Material::Sand => {
                fall(x, y, grid);
            }
            Material::Water => {
                if !fall(x, y, grid) {
                    flow(x, y, grid);
                }
            }
            Material::Concrete => {
                // Solid: Does not move
            }
            Material::Smoke => {
                if !rise(x, y, grid) {
                    flow(x, y, grid);
                }
            }
            Material::Empty => {
                // No behavior
            }
        }
    }
}

fn fall(x: usize, y: usize, grid: &mut Grid) -> bool {
    if y < config::GRID_HEIGHT - 1 && grid.get(x, y + 1) == Material::Empty as u8 {
        grid.move_to(x, y, x, y + 1);
        return true;
    }

    if y < config::GRID_HEIGHT - 1 {
        let left = x > 0 && grid.get(x - 1, y + 1) == Material::Empty as u8;
        let right = x < config::GRID_WIDTH - 1 && grid.get(x + 1, y + 1) == Material::Empty as u8;
        if left && right {
            if rand::random() {
                grid.move_to(x, y, x - 1, y + 1);
            } else {
                grid.move_to(x, y, x + 1, y + 1);
            }
            return true;
        } else if left {
            grid.move_to(x, y, x - 1, y + 1);
            return true;
        } else if right {
            grid.move_to(x, y, x + 1, y + 1);
            return true;
        }
    }
    false
}

fn flow(x: usize, y: usize, grid: &mut Grid) {
    let left = x > 0 && grid.get(x - 1, y) == Material::Empty as u8;
    let right = x < config::GRID_WIDTH - 1 && grid.get(x + 1, y) == Material::Empty as u8;
    if left && right {
        if rand::random() {
            grid.move_to(x, y, x - 1, y);
        } else {
            grid.move_to(x, y, x + 1, y);
        }
    } else if left {
        grid.move_to(x, y, x - 1, y);
    } else if right {
        grid.move_to(x, y, x + 1, y);
    }
}

fn rise(x: usize, y: usize, grid: &mut Grid) -> bool {
    if y > 0 && grid.get(x, y - 1) == Material::Empty as u8 {
        grid.move_to(x, y, x, y - 1);
        return true;
    }
    if y > 0 {
        let left = x > 0 && grid.get(x - 1, y - 1) == Material::Empty as u8;
        let right = x < config::GRID_WIDTH - 1 && grid.get(x + 1, y - 1) == Material::Empty as u8;
        if left && right {
            if rand::random() {
                grid.move_to(x, y, x - 1, y - 1);
            } else {
                grid.move_to(x, y, x + 1, y - 1);
            }
            return true;
        } else if left {
            grid.move_to(x, y, x - 1, y - 1);
            return true;
        } else if right {
            grid.move_to(x, y, x + 1, y - 1);
            return true;
        }
    }
    false
}