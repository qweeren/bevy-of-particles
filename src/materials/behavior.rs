use crate::{grid::Grid, utils::grid_utils::find_vertical_space};
use crate::materials::Material;
use crate::config;
use rand::{prelude::*, rng};
use crate::utils::grid_utils::find_horizontal_space;

const GRAVITY: f32 = 0.5;
const MAX_FALL_SPEED: f32 = 8.0;
const BOUNCE_FACTOR: f32 = 0.3;
const MIN_MOVEMENT_THRESHOLD: f32 = 0.1;

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
    let mut current_velocity = old_grid.get_velocity(x, y);
    
    // Always apply gravity if there's space below
    if y < config::GRID_HEIGHT - 1 && old_grid.get(x, y + 1) == Material::Empty as u8 {
        current_velocity += GRAVITY;
    } else if current_velocity < MIN_MOVEMENT_THRESHOLD {
        // If blocked and nearly stopped, fully stop
        current_velocity = 0.0;
        new_grid.set(x, y, old_grid.get(x, y));
        new_grid.set_velocity(x, y, current_velocity);
        return false;
    }

    current_velocity = current_velocity.min(MAX_FALL_SPEED);

    // Convert velocity to discrete steps, but always move at least 1 if there's any velocity
    let fall_distance = (current_velocity.abs().round() as usize).max(1);

    let bottom_y = find_vertical_space(old_grid, x, y, fall_distance);
    
    if bottom_y == y {
        // Hit an obstacle, check for diagonal movement
        if current_velocity > 1.0 {
            // Try to slide diagonally
            let left_clear = x > 0 && old_grid.get(x - 1, y) == Material::Empty as u8;
            let right_clear = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y) == Material::Empty as u8;
            
            match (left_clear, right_clear) {
                (true, true) => {
                    // Randomly choose direction
                    if rng().gen::<bool>() {
                        new_grid.move_to_with_velocity(x, y, x - 1, y);
                        new_grid.set_velocity(x - 1, y, current_velocity * 0.8);
                    } else {
                        new_grid.move_to_with_velocity(x, y, x + 1, y);
                        new_grid.set_velocity(x + 1, y, current_velocity * 0.8);
                    }
                    return true;
                }
                (true, false) => {
                    new_grid.move_to_with_velocity(x, y, x - 1, y);
                    new_grid.set_velocity(x - 1, y, current_velocity * 0.8);
                    return true;
                }
                (false, true) => {
                    new_grid.move_to_with_velocity(x, y, x + 1, y);
                    new_grid.set_velocity(x + 1, y, current_velocity * 0.8);
                    return true;
                }
                _ => {
                    // Bounce with reduced velocity
                    current_velocity *= -BOUNCE_FACTOR;
                    if current_velocity.abs() < MIN_MOVEMENT_THRESHOLD {
                        current_velocity = 0.0;
                    }
                    new_grid.set(x, y, old_grid.get(x, y));
                    new_grid.set_velocity(x, y, current_velocity);
                    return false;
                }
            }
        } else {
            // Too slow for diagonal movement, just stop
            new_grid.set(x, y, old_grid.get(x, y));
            new_grid.set_velocity(x, y, 0.0);
            return false;
        }
    }

    // Move to the new position
    new_grid.move_to_with_velocity(x, y, x, bottom_y);
    new_grid.set_velocity(x, bottom_y, current_velocity);
    true
}

fn flow(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) {
    let mut rng = rng();
    let current_material = Material::from_id(old_grid.get(x, y));
    let viscosity = current_material.properties().viscosity;
    
    // Calculate max movement distance based on viscosity (1.0 = no movement, 0.0 = max movement)
    let max_distance = ((1.0 - viscosity) * 5.0).round() as usize;
    if max_distance == 0 || rng.gen::<f32>() <= viscosity {
        new_grid.set(x, y, old_grid.get(x, y));
        return;
    }

    // Find available space
    let (left_bound, right_bound) = find_horizontal_space(old_grid, x, y, max_distance);
    
    // Calculate actual movement
    if left_bound == x && right_bound == x {
        // Nowhere to move
        new_grid.set(x, y, old_grid.get(x, y));
        return;
    }

    // Determine movement distance and direction
    let total_space = right_bound - left_bound;
    if total_space > 0 {
        let mut target_x = x;

        if left_bound < x && right_bound > x {
            // Can move both directions
            if rng.gen::<bool>() {
                // Move left - prefer maximum movement
                let max_left = x - left_bound;
                let move_amount = if rng.gen::<f32>() < 0.7 {
                    max_left  // 70% chance to move maximum distance
                } else {
                    rng.gen_range(1..=max_left)
                };
                target_x = x - move_amount;
            } else {
                // Move right - prefer maximum movement
                let max_right = right_bound - x;
                let move_amount = if rng.gen::<f32>() < 0.7 {
                    max_right  // 70% chance to move maximum distance
                } else {
                    rng.gen_range(1..=max_right)
                };
                target_x = x + move_amount;
            }
        } else if left_bound < x {
            // Can only move left
            let max_left = x - left_bound;
            let move_amount = if rng.gen::<f32>() < 0.7 {
                max_left
            } else {
                rng.gen_range(1..=max_left)
            };
            target_x = x - move_amount;
        } else if right_bound > x {
            // Can only move right
            let max_right = right_bound - x;
            let move_amount = if rng.gen::<f32>() < 0.7 {
                max_right
            } else {
                rng.gen_range(1..=max_right)
            };
            target_x = x + move_amount;
        }

        if target_x != x {
            new_grid.move_to(x, y, target_x, y);
            return;
        }
    }

    new_grid.set(x, y, old_grid.get(x, y));
}

