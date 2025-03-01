use bevy::prelude::*;
use crate::config::{self, CAMERA_OFFSET_X, CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::grid::Grid;
use crate::materials::Material;

/// Converts window coordinates to grid coordinates
/// Returns None if the coordinates are outside the grid bounds
pub fn get_grid_pos(window: &Window, camera: &Camera, camera_transform: &GlobalTransform) -> Option<(usize, usize)> {
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            let adjusted_x = world_pos.x + (GRID_WIDTH as f32 * CELL_SIZE / 2.0);
            let adjusted_y = world_pos.y + (GRID_HEIGHT as f32 * CELL_SIZE / 2.0);
            let grid_x = ((adjusted_x + CAMERA_OFFSET_X) / CELL_SIZE) as usize;
            let grid_y = (GRID_HEIGHT as f32 - adjusted_y / CELL_SIZE) as usize;
            
            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                return Some((grid_x, grid_y));
            }
        }
    }
    None
}

/// Finds the maximum distance a particle can move horizontally before hitting an obstacle
/// Returns the maximum possible x coordinate in both directions (left_x, right_x)
pub fn find_horizontal_space(grid: &Grid, x: usize, y: usize, max_distance: usize) -> (usize, usize) {
    let mut left_x = x;
    let mut right_x = x;
    
    // Check both directions simultaneously
    for dx in 1..=max_distance {
        let mut check_left = x >= dx;
        let mut check_right = x + dx < config::GRID_WIDTH;
        
        if !check_left && !check_right {
            break;
        }

        // Check left if possible
        if check_left && grid.get(x - dx, y).material_type == Material::Empty as u8 {
            left_x = x - dx;
        } else {
            // Stop checking left if blocked
            check_left = false;
        }

        // Check right if possible
        if check_right && grid.get(x + dx, y).material_type == Material::Empty as u8 {
            right_x = x + dx;
        } else {
            // Stop checking right if blocked
            check_right = false;
        }

        // Break if both directions are blocked
        if !check_left && !check_right {
            break;
        }
    }
    
    (left_x, right_x)
}

/// Finds the maximum distance a particle can move vertically before hitting an obstacle
/// Returns the maximum possible y coordinate in downward direction
pub fn find_vertical_space(grid: &Grid, x: usize, y: usize, max_distance: usize) -> usize {
    // Early return if already at bottom or max_distance is 0
    if y >= config::GRID_HEIGHT - 1 || max_distance == 0 {
        return y;
    }

    // Calculate the maximum possible distance considering grid bounds
    let max_check = (max_distance + y).min(config::GRID_HEIGHT - 1) - y;
    
    // Use iterator to find first non-empty cell
    match (1..=max_check)
        .take_while(|&dy| grid.get(x, y + dy).material_type == Material::Empty as u8)
        .last()
    {
        Some(last_empty) => y + last_empty,
        None => y
    }
}
