use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::config::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE};
use crate::grid::Grid;
use crate::utils::{line::bresenham_line, grid_utils::get_grid_pos};

use super::input::{Drawing, LastMouseGridPos};
use super::resources::{BrushSize, SelectedMaterial};

fn place_material_with_brush(grid: &mut Grid, x: usize, y: usize, material: u8, brush_size: usize) {
    let half_size = brush_size as isize / 2;
    let radius_sq = half_size * half_size;
    for dx in -half_size..=half_size {
        for dy in -half_size..=half_size {
            if dx * dx + dy * dy <= radius_sq {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                // Only place material if the position is in bounds AND the cell is empty
                if grid.in_bounds(nx, ny) && grid.get(nx as usize, ny as usize) == 0 {
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
    mut last_pos: ResMut<LastMouseGridPos>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();

    match (buttons.pressed(MouseButton::Left), drawing.0) {
        (true, false) => {
            drawing.0 = true;
            if let Some(pos) = get_grid_pos(window, camera, camera_transform) {
                place_material_with_brush(&mut grid, pos.0, pos.1, selected_material.0, brush_size.0.into());
                last_pos.0 = Some(pos);
            }
        }
        (true, true) => {
            if let Some(current_pos) = get_grid_pos(window, camera, camera_transform) {
                place_material_with_brush(&mut grid, current_pos.0, current_pos.1, selected_material.0, brush_size.0.into());
                
                if let Some(last) = last_pos.0 {
                    let line_points = bresenham_line(last.0, last.1, current_pos.0, current_pos.1);
                    for (x, y) in line_points {
                        place_material_with_brush(&mut grid, x, y, selected_material.0, brush_size.0.into());
                    }
                }
                last_pos.0 = Some(current_pos);
            }
        }
        (false, _) => {
            drawing.0 = false;
            last_pos.0 = None;
        }
    }
}
