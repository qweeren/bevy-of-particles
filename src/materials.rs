
use bevy::color::Color;

use crate::grid::{self, Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Empty = 0,    // No material
    Sand = 1,     // Powder type
    Water = 2,    // Liquid type
    Concrete = 3, // Solid type
    Smoke = 4,    // Gas type
}

// Material falls downward if the space below is empty
fn fall(x: usize, y: usize, grid: &mut Grid) -> bool {
    if y < grid::GRID_HEIGHT - 1 && grid.get(x, y + 1) == Material::Empty as u8 {
        grid.move_to(x, y, x, y + 1);
        return true; // Successfully moved
    }
    false // Blocked or at bottom
}

// Material flows left or right if the space below is blocked
fn flow(x: usize, y: usize, grid: &mut Grid) {
    let left = x > 0 && grid.get(x - 1, y) == Material::Empty as u8;
    let right = x < grid::GRID_WIDTH - 1 && grid.get(x + 1, y) == Material::Empty as u8;
    if left && right {
        // Randomly choose left or right
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

// Material rises upward if the space above is empty
fn rise(x: usize, y: usize, grid: &mut Grid) -> bool {
    if y > 0 && grid.get(x, y - 1) == Material::Empty as u8 {
        grid.move_to(x, y, x, y - 1);
        return true; // Successfully moved
    }
    false // Blocked or at top
}

pub trait MaterialBehavior {
    fn update(&self, x: usize, y: usize, grid: &mut Grid);
}

impl MaterialBehavior for Material {
    fn update(&self, x: usize, y: usize, grid: &mut Grid) {
        match self {
            Material::Sand => {
                // Powder: Falls downward
                fall(x, y, grid);
            }
            Material::Water => {
                // Liquid: Falls downward, flows if blocked
                if !fall(x, y, grid) {
                    flow(x, y, grid);
                }
            }
            Material::Concrete => {
                // Solid: Does not move (no behavior)
            }
            Material::Smoke => {
                // Gas: Rises upward, flows if blocked
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