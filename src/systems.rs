// src/systems.rs

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use crate::grid::{Grid, GRID_WIDTH, GRID_HEIGHT, CELL_SIZE};
use crate::materials::MaterialBehavior;
use crate::registry::Material;
use rayon::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2d {
        ..default()
    });

    // Create a texture for the grid
    let size = Extent3d {
        width: GRID_WIDTH as u32,
        height: GRID_HEIGHT as u32,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new(
        size,
        TextureDimension::D2,
        vec![0; GRID_WIDTH * GRID_HEIGHT * 4], // RGBA, 4 bytes per pixel
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    // Initialize to black
    for pixel in image.data.chunks_exact_mut(4) {
        pixel[0] = 0;   // R
        pixel[1] = 0;   // G
        pixel[2] = 0;   // B
        pixel[3] = 255; // A
    }

    let image_handle = images.add(image);
    let window = window_query.get_single().unwrap();
    // Spawn a sprite with the texture
    commands.spawn((
        Sprite {
            image: image_handle.clone(),
            custom_size: Some(Vec2::new(
                (GRID_WIDTH as f32) * CELL_SIZE,
                (GRID_HEIGHT as f32) * CELL_SIZE,
            )),
            anchor: Anchor::Custom(Vec2::new(-0.5, 0.0)),
            ..Default::default()
        },
        Transform::from_xyz(-window.width() / 2.0, 0.0, 0.0),
    ));

    // Store the texture handle as a resource
    commands.insert_resource(SimulationTexture { image_handle });
}

#[derive(Resource)]
pub struct SimulationTexture {
    image_handle: Handle<Image>,
}

pub fn update_grid(mut grid: ResMut<Grid>) {
    let mut new_grid = grid.clone();
    
    // Process in chunks of rows to maintain local consistency
    for chunk_y in (0..GRID_HEIGHT).step_by(3).rev() {
        // Process 3 rows at a time to handle diagonal movements
        let chunk_end = (chunk_y + 3).min(GRID_HEIGHT);
        
        for y in chunk_y..chunk_end {
            for x in 0..GRID_WIDTH {
                let material = grid.get(x, y);
                if material != Material::Empty as u8 {
                    let material_enum = match material {
                        1 => Material::Sand,
                        2 => Material::Water,
                        3 => Material::Concrete,
                        4 => Material::Smoke,
                        _ => Material::Empty,
                    };
                    material_enum.update(x, y, &mut new_grid);
                }
            }
        }
    }
    
    *grid = new_grid;
}

pub fn render_grid(
    grid: Res<Grid>,
    sim_texture: Res<SimulationTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(image) = images.get_mut(&sim_texture.image_handle) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let material = grid.get(x, y);
                let material_enum = match material {
                    0 => Material::Empty,
                    1 => Material::Sand,
                    2 => Material::Water,
                    3 => Material::Concrete,
                    4 => Material::Smoke,
                    _ => Material::Empty,
                };
                let color = material_enum.color();
                let [r, g, b, _] = color.to_srgba().to_u8_array();
                let pixel_index = (x + y * GRID_WIDTH) * 4;

                image.data[pixel_index] = r;
                image.data[pixel_index + 1] = g;
                image.data[pixel_index + 2] = b;
                image.data[pixel_index + 3] = 255; // Full opacity
            }
        }
    }
}

pub fn spawn_test_materials(mut grid: ResMut<Grid>) {
    grid.set(50, 10, Material::Sand as u8);
    grid.set(40, 20, Material::Water as u8);
    grid.set(60, 30, Material::Concrete as u8);
    grid.set(50, 40, Material::Smoke as u8);
}
