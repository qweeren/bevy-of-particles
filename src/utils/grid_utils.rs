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
    
    // Check left
    for dx in 1..=max_distance {
        if x < dx || grid.get(x - dx, y).material_type != Material::Empty as u8 {
            break;
        }
        left_x = x - dx;
    }
    
    // Check right
    for dx in 1..=max_distance {
        if x + dx >= config::GRID_WIDTH || grid.get(x + dx, y).material_type != Material::Empty as u8 {
            break;
        }
        right_x = x + dx;
    }
    
    (left_x, right_x)
}

/// Finds the maximum distance a particle can move vertically before hitting an obstacle
/// Returns the maximum possible y coordinate in downward direction
pub fn find_vertical_space(grid: &Grid, x: usize, y: usize, max_distance: usize) -> usize {
    let mut bottom_y = y;
    
    // Check downward
    for dy in 1..=max_distance {
        if y + dy >= config::GRID_HEIGHT || grid.get(x, y + dy).material_type != Material::Empty as u8 {
            break;
        }
        bottom_y = y + dy;
    }
    
    bottom_y
}
