use bevy::prelude::*;
use crate::config::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, CAMERA_OFFSET_X};

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