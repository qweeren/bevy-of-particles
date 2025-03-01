use crate::{grid::Grid, utils::grid_utils::find_vertical_space};
use crate::materials::{Material, types::ParticleFlags};
use crate::config;
use rand::{prelude::*, rng};
use crate::utils::grid_utils::find_horizontal_space;

use super::types::Particle;

const GRAVITY: f32 = 0.5;
const MAX_FALL_SPEED: f32 = 8.0;
const BOUNCE_FACTOR: f32 = 0.3;
const MIN_MOVEMENT_THRESHOLD: f32 = 0.1;
const FIRE_DISSIPATION_CHANCE: f32 = 0.4; // 10% for connected fire
const FIRE_ISOLATED_DISSIPATION_CHANCE: f32 = 0.7; // 50% for isolated fire
const FIRE_SPREAD_CHANCE: f32 = 0.3;      // 20% chance to attempt spread
const FIRE_COLORS: [(u8, u8, u8); 4] = [
    (255, 100, 0),   // Orange
    (255, 60, 0),    // Bright orange
    (255, 160, 30),  // Yellow-orange
    (255, 50, 0),    // Deep orange
];
const SMOKE_DISSIPATION_CHANCE: f32 = 0.1; // 30% chance to disappear
const FIRE_TO_SMOKE_CHANCE: f32 = 0.2; // 30% chance for dissipating fire to become smoke
pub trait MaterialBehavior {
    fn update(&self, x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid);
}

impl MaterialBehavior for Material {
    fn update(&self, x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) {
        // Skip if already updated in this frame
        if new_grid.get(x, y).material_type != old_grid.get(x, y).material_type {
            return;
        }

        let particle = old_grid.get(x, y);

        // Handle fire behavior separately
        if particle.material_type == Material::Fire as u8 {
            if handle_fire(x, y, new_grid, old_grid) {
                return;
            }
        }

        // Handle smoke dissipation
        if particle.material_type == Material::Smoke as u8 {
            if rng().gen::<f32>() < SMOKE_DISSIPATION_CHANCE {
                new_grid.set(x, y, Material::Empty);
                return;
            }
        }

        // If not movable, just copy the state
        if !particle.flags.contains(ParticleFlags::MOVABLE) {
            new_grid.set(x, y, *self);
            return;
        }

        // Remaining logic for other materials (unchanged)
        if try_move_density_based(x, y, new_grid, old_grid) {
            return;
        }
        if particle.flags.contains(ParticleFlags::RISES) {
            if rise(x, y, new_grid, old_grid) {
                return;
            }
        } else {
            if fall(x, y, new_grid, old_grid) {
                return;
            }
        }
        if particle.flags.contains(ParticleFlags::FLOWS) {
            flow(x, y, new_grid, old_grid);
            return;
        }
        new_grid.set(x, y, *self);
    }
}

fn try_move_density_based(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) -> bool {
    let current_particle = old_grid.get(x, y);
    let current_density = current_particle.get_density();

    let mut check_and_swap = |x2: usize, y2: usize| -> bool {
        let other_particle = old_grid.get(x2, y2);
        
        // Only interact with non-empty particles
        if other_particle.material_type != Material::Empty as u8 {
            let other_density = other_particle.get_density();
            if current_density > other_density {
                new_grid.swap(x, y, x2, y2);
                return true;
            }
        }
        false
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
    if y > 0 && old_grid.get(x, y - 1).material_type == Material::Empty as u8 {
        new_grid.move_to(x, y, x, y - 1);
        return true;
    }
    if y > 0 {
        let left = x > 0 && old_grid.get(x - 1, y - 1).material_type == Material::Empty as u8;
        let right = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y - 1).material_type == Material::Empty as u8;
        
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
    let current_particle = old_grid.get(x, y);
    
    // Always apply gravity if there's space below
    if y < config::GRID_HEIGHT - 1 && old_grid.get(x, y + 1).material_type == Material::Empty as u8 {
        current_velocity += GRAVITY;
    } else if current_velocity < MIN_MOVEMENT_THRESHOLD {
        // If blocked and nearly stopped, fully stop
        current_velocity = 0.0;
        new_grid.set(x, y, Material::from_id(current_particle.material_type));
        new_grid.set_velocity(x, y, current_velocity);
        return false;
    }

    current_velocity = current_velocity.min(MAX_FALL_SPEED);

    // Transfer velocity to nearby particles of the same type
    if current_velocity > 1.0 {
        let transfer_amount = current_velocity * 0.2; // Transfer 20% of velocity
        
        // Check adjacent cells
        let check_positions = [
            (x.wrapping_sub(1), y), // Left
            (x + 1, y),            // Right
            (x, y.wrapping_sub(1)), // Up
            (x + 1, y.wrapping_sub(1)), // Up-Right
            (x.wrapping_sub(1), y.wrapping_sub(1)), // Up-Left
            (x, y + 1),            // Down
            (x + 1, y + 1),        // Down-Right
            (x.wrapping_sub(1), y + 1), // Down-Left
        ];

        for (nx, ny) in check_positions {
            if nx < config::GRID_WIDTH && ny < config::GRID_HEIGHT {
                let neighbor = old_grid.get(nx, ny);
                if neighbor.material_type == current_particle.material_type {
                    let neighbor_velocity = old_grid.get_velocity(nx, ny);
                    if neighbor_velocity < current_velocity {
                        new_grid.set_velocity(nx, ny, neighbor_velocity + transfer_amount);
                    }
                }
            }
        }
        
        current_velocity *= 0.8;
    }

    let fall_distance = (current_velocity.abs().round() as usize).max(1);
    let bottom_y = find_vertical_space(old_grid, x, y, fall_distance);
    
    if bottom_y > y + 1 {
        current_velocity += GRAVITY * ((bottom_y - y) as f32);
        current_velocity = current_velocity.min(MAX_FALL_SPEED);
    }
    
    if bottom_y == y {
        if current_velocity > 1.0 {
            let left_clear = x > 0 && old_grid.get(x - 1, y).material_type == Material::Empty as u8;
            let right_clear = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y).material_type == Material::Empty as u8;
            
            match (left_clear, right_clear) {
                (true, true) => {
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
                    current_velocity *= -BOUNCE_FACTOR;
                    if current_velocity.abs() < MIN_MOVEMENT_THRESHOLD {
                        current_velocity = 0.0;
                    }
                    new_grid.set(x, y, Material::from_id(current_particle.material_type));
                    new_grid.set_velocity(x, y, current_velocity);
                    return false;
                }
            }
        } else {
            new_grid.set(x, y, Material::from_id(current_particle.material_type));
            new_grid.set_velocity(x, y, 0.0);
            return false;
        }
    }

    new_grid.move_to_with_velocity(x, y, x, bottom_y);
    new_grid.set_velocity(x, bottom_y, current_velocity);
    true
}

fn flow(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) {
    let mut rng = rng();
    let current_particle = old_grid.get(x, y);
    let viscosity = current_particle.get_viscosity();
    
    let max_distance = ((1.0 - viscosity) * 5.0).round() as usize;
    if max_distance == 0 || rng.gen::<f32>() <= viscosity {
        new_grid.set(x, y, Material::from_id(current_particle.material_type));
        return;
    }

    let (left_bound, right_bound) = find_horizontal_space(old_grid, x, y, max_distance);
    
    if left_bound == x && right_bound == x {
        new_grid.set(x, y, Material::from_id(current_particle.material_type));
        return;
    }

    let total_space = right_bound - left_bound;
    if total_space > 0 {
        let mut target_x = x;

        if left_bound < x && right_bound > x {
            if rng.gen::<bool>() {
                let max_left = x - left_bound;
                let move_amount = if rng.gen::<f32>() < 0.7 {
                    max_left
                } else {
                    rng.gen_range(1..=max_left)
                };
                target_x = x - move_amount;
            } else {
                let max_right = right_bound - x;
                let move_amount = if rng.gen::<f32>() < 0.7 {
                    max_right
                } else {
                    rng.gen_range(1..=max_right)
                };
                target_x = x + move_amount;
            }
        } else if left_bound < x {
            let max_left = x - left_bound;
            let move_amount = if rng.gen::<f32>() < 0.7 {
                max_left
            } else {
                rng.gen_range(1..=max_left)
            };
            target_x = x - move_amount;
        } else if right_bound > x {
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

    new_grid.set(x, y, Material::from_id(current_particle.material_type));
}

fn has_fire_neighbors(x: usize, y: usize, grid: &Grid) -> bool {
    let directions = [
        (-1, -1), (0, -1), (1, -1), // Above
        (-1, 0),           (1, 0),  // Sides
        (-1, 1),  (0, 1),  (1, 1), // Below
    ];

    for (dx, dy) in directions.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < config::GRID_WIDTH as isize &&
           new_y >= 0 && new_y < config::GRID_HEIGHT as isize {
            let neighbor = grid.get(new_x as usize, new_y as usize);
            if neighbor.material_type == Material::Fire as u8 {
                return true;
            }
        }
    }
    false
}

fn handle_fire(x: usize, y: usize, new_grid: &mut Grid, old_grid: &Grid) -> bool {
    let mut rng = rng();

    // Determine dissipation chance based on isolation
    let is_isolated = !has_fire_neighbors(x, y, old_grid);
    let dissipation_chance = if is_isolated {
        FIRE_ISOLATED_DISSIPATION_CHANCE // 50% for isolated
    } else {
        FIRE_DISSIPATION_CHANCE          // 10% for connected
    };

    // Handle dissipation with chance to create smoke
    if rng.gen::<f32>() < dissipation_chance {
        if rng.gen::<f32>() < FIRE_TO_SMOKE_CHANCE {
            new_grid.set(x, y, Material::Smoke);
        } else {
            new_grid.set(x, y, Material::Empty);
        }
        return true;
    }

    // Preserve existing upward movement logic
    let upward_bias = if y > 0 {
        let above = old_grid.get(x, y - 1).material_type == Material::Empty as u8;
        let above_left = x > 0 && old_grid.get(x - 1, y - 1).material_type == Material::Empty as u8;
        let above_right = x < config::GRID_WIDTH - 1 && old_grid.get(x + 1, y - 1).material_type == Material::Empty as u8;
        
        match (above, above_left, above_right) {
            (true, _, _) => 0.8,    // Strongly prefer moving straight up
            (false, true, true) => 0.6,  // Prefer diagonal movement
            (false, true, false) => 0.4,  // Slight left bias
            (false, false, true) => 0.4,  // Slight right bias
            _ => 0.2,    // Limited movement options
        }
    } else {
        0.0
    };

    // Randomize fire color
    let color_idx = (rng.gen::<f32>() * FIRE_COLORS.len() as f32) as usize;
    let (r, g, b) = FIRE_COLORS[color_idx];
    let new_particle = Particle::new(Material::Fire).with_color(r, g, b);

    // Handle upward movement
    if rng.gen::<f32>() < upward_bias {
        if y > 0 {
            let dx = if rng.gen::<f32>() < 0.3 {
                if rng.gen::<bool>() { 1 } else { -1 } // 30% chance to move diagonally
            } else {
                0 // 70% chance to move straight up
            };
            let new_x = (x as isize + dx) as usize;
            if new_x > 0 && new_x < config::GRID_WIDTH {
                new_grid.set_particle(new_x, y - 1, new_particle);
                return true;
            }
        }
    }

    // Spread fire with directional probabilities
    if rng.gen::<f32>() < FIRE_SPREAD_CHANCE {
        let spread_directions = [
            (0, -1, 0.5),  // Up: 50%
            (-1, -1, 0.4), // Up-left: 40%
            (1, -1, 0.4),  // Up-right: 40%
            (-1, 0, 0.2),  // Left: 20%
            (1, 0, 0.2),   // Right: 20%
        ];

        for (dx, dy, prob) in spread_directions.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            
            if new_x >= 0 && new_x < config::GRID_WIDTH as isize &&
               new_y >= 0 && new_y < config::GRID_HEIGHT as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                
                if old_grid.get(new_x, new_y).material_type == Material::Empty as u8 {
                    if rng.gen::<f32>() < *prob {
                        new_grid.set_particle(new_x, new_y, new_particle);
                    }
                }
            }
        }
    }

    // Maintain fire at current position if no movement/spread occurs
    new_grid.set_particle(x, y, new_particle);
    true
}





































