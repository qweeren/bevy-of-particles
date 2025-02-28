use bevy::prelude::*;
use crate::config;
use crate::grid::{self, Grid};
use crate::materials::Material;
use super::SimulationTexture;

pub fn render_grid(
    grid: Res<Grid>,
    simulation_texture: Res<SimulationTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(image) = images.get_mut(&simulation_texture.image_handle) {
        for y in 0..config::GRID_HEIGHT {
            for x in 0..config::GRID_WIDTH {
                let material_id = grid.get(x, y);
                let material = crate::registry::material_from_id(material_id);
                let properties = material.properties();
                let pixel_index = (y * config::GRID_WIDTH + x) * 4;
                
                image.data[pixel_index] = properties.color.0;     // R
                image.data[pixel_index + 1] = properties.color.1; // G
                image.data[pixel_index + 2] = properties.color.2; // B
                image.data[pixel_index + 3] = 255;               // A
            }
        }
    }
}
