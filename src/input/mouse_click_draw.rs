use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{grid::{Grid, CELL_SIZE, GRID_HEIGHT, GRID_WIDTH}, utils::line::bresenham_line};

use super::input::{BrushSize, Drawing, LastMouseGridPos, SelectedMaterial};

fn place_material_with_brush(grid: &mut Grid, x: usize, y: usize, material: u8, brush_size: usize) {
    let half_size = brush_size as isize / 2;
    let radius_sq = half_size * half_size;
    for dx in -half_size..=half_size {
        for dy in -half_size..=half_size {
            if dx * dx + dy * dy <= radius_sq {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if grid.in_bounds(nx, ny) {
                    grid.set(nx as usize, ny as usize, material);
                }
            }
        }
    }
}

pub fn mouse_click_draw(
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_material: Res<SelectedMaterial>,
    mut grid: ResMut<Grid>,
    mut drawing: ResMut<Drawing>,
    brush_size: Res<BrushSize>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();

    // Toggle drawing mode
    if buttons.just_pressed(MouseButton::Left) {
        drawing.0 = true;
    }
    if buttons.just_released(MouseButton::Left) {
        drawing.0 = false;
    }

    // Draw continuously while active
    if drawing.0 {
        if let Some(cursor_pos) = window.cursor_position() {
            match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                Ok(world_pos) => {
                    // Adjust for camera centering (500, 500)
                    let adjusted_x = world_pos.x + (GRID_WIDTH as f32 * CELL_SIZE / 2.0);
                    let adjusted_y = world_pos.y + (GRID_HEIGHT as f32 * CELL_SIZE / 2.0);
                    let grid_x = ((adjusted_x + 115.0) / CELL_SIZE)as usize;
                    let grid_y = (GRID_HEIGHT as f32 - adjusted_y / CELL_SIZE) as usize;

                    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                        place_material_with_brush(&mut grid, grid_x, grid_y, selected_material.0, brush_size.0);
                    }
                }
                Err(_) => {
                    // Do nothing if conversion fails
                }
            }
        }
    }
}