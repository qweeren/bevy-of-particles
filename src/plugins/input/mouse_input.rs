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
    
    // Pre-calculate the radius check results
    let radius_mask: Vec<Vec<bool>> = (-half_size..=half_size)
        .map(|dx| (-half_size..=half_size)
            .map(|dy| dx * dx + dy * dy <= radius_sq)
            .collect())
        .collect();
    
    // First pass: check if we're placing on top of existing particles
    let mut should_add_velocity = false;
    'outer: for (i, dx) in (-half_size..=half_size).enumerate() {
        for (j, dy) in (-half_size..=half_size).enumerate() {
            if !radius_mask[i][j] { continue; }
            
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if grid.in_bounds(nx, ny) {
                let ny = ny as usize;
                if ny < config::GRID_HEIGHT - 1 && 
                   grid.get(nx as usize, ny + 1) != Material::Empty as u8 {
                    should_add_velocity = true;
                    break 'outer;
                }
            }
        }
    }

    // Second pass: place particles
    for (i, dx) in (-half_size..=half_size).enumerate() {
        for (j, dy) in (-half_size..=half_size).enumerate() {
            if !radius_mask[i][j] { continue; }
            
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if grid.in_bounds(nx, ny) {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid.get(nx, ny) == Material::Empty as u8 {
                    grid.set(nx, ny, material);
                    grid.set_velocity(nx, ny, if should_add_velocity { 1.0 } else { 0.0 });
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
    // Handle scroll events in batch
    let scroll_delta: f32 = scroll_evr.read().map(|ev| match ev.unit {
        MouseScrollUnit::Line => ev.y,
        MouseScrollUnit::Pixel => ev.y / 20.0,
    }).sum();
    
    if scroll_delta != 0.0 {
        brush_size.0 = ((brush_size.0 as f32 + scroll_delta * BRUSH_SIZE_SCROLL_STEP as f32)
            .clamp(1.0, 30.0)) as u8;
    }

    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();

    // Only calculate grid position if needed
    if !buttons.pressed(MouseButton::Left) {
        drawing.0 = false;
        last_pos.0 = None;
        return;
    }

    let Some(current_pos) = get_grid_pos(window, camera, camera_transform) else { return };

    if !drawing.0 {
        // First click
        drawing.0 = true;
        place_material_with_brush(&mut grid, current_pos.0, current_pos.1, selected_material.0, brush_size.0.into());
        last_pos.0 = Some(current_pos);
        return;
    }

    // Continuous drawing
    place_material_with_brush(&mut grid, current_pos.0, current_pos.1, selected_material.0, brush_size.0.into());
    
    if let Some(last) = last_pos.0 {
        // Use itertools for more efficient iteration
        for (x, y) in bresenham_line(last.0, last.1, current_pos.0, current_pos.1) {
            place_material_with_brush(&mut grid, x, y, selected_material.0, brush_size.0.into());
        }
    }
    last_pos.0 = Some(current_pos);
}
