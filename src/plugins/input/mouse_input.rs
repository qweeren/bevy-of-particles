use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::config;
use crate::grid::Grid;
use crate::utils::{line::bresenham_line, grid_utils::get_grid_pos};
use crate::materials::Material;

use super::input::{Drawing, LastMouseGridPos};
use super::resources::{BrushSize, SelectedMaterial};

const BRUSH_SIZE_SCROLL_STEP: u8 = 1;

fn place_material_with_brush(grid: &mut Grid, x: usize, y: usize, material: u8, brush_size: usize) {
    let half_size = brush_size as isize / 2;
    let radius_sq = half_size * half_size;
    
    // First pass: check if we're placing on top of existing particles
    let mut should_add_velocity = false;
    for dx in -half_size..=half_size {
        for dy in -half_size..=half_size {
            if dx * dx + dy * dy <= radius_sq {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if grid.in_bounds(nx, ny) {
                    let ny = ny as usize;
                    // If there's a particle below, we should add initial velocity
                    if ny < config::GRID_HEIGHT - 1 && 
                       grid.get(nx as usize, ny + 1) != Material::Empty as u8 {
                        should_add_velocity = true;
                        break;
                    }
                }
            }
        }
        if should_add_velocity {
            break;
        }
    }

    // Second pass: place particles with appropriate velocity
    for dx in -half_size..=half_size {
        for dy in -half_size..=half_size {
            if dx * dx + dy * dy <= radius_sq {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if grid.in_bounds(nx, ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    // Only place material if the position is in bounds AND the cell is empty
                    if grid.get(nx, ny) == Material::Empty as u8 {
                        grid.set(nx, ny, material);
                        // Add initial velocity if we're placing on top of other particles
                        if should_add_velocity {
                            grid.set_velocity(nx, ny, 1.0);
                        } else {
                            grid.set_velocity(nx, ny, 0.0);
                        }
                    }
                }
            }
        }
    }
}

pub fn mouse_click_draw(
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_material: Res<SelectedMaterial>,
    mut grid: ResMut<Grid>,
    mut drawing: ResMut<Drawing>,
    mut brush_size: ResMut<BrushSize>,
    mut last_pos: ResMut<LastMouseGridPos>,
) {
    // Handle scroll events for brush size
    for ev in scroll_evr.read() {
        let scroll_amount = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y / 20.0, // Adjust sensitivity for pixel-based scrolling
        };
        
        if scroll_amount > 0.0 {
            brush_size.0 = (brush_size.0 + BRUSH_SIZE_SCROLL_STEP).min(30);
        } else if scroll_amount < 0.0 {
            brush_size.0 = (brush_size.0.saturating_sub(BRUSH_SIZE_SCROLL_STEP)).max(1);
        }
    }

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
