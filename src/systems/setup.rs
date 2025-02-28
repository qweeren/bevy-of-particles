use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE};

#[derive(Resource)]
pub struct SimulationTexture {
    pub(crate) image_handle: Handle<Image>,
}

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_camera(&mut commands);
    let image_handle = create_grid_texture(&mut images);
    spawn_grid_sprite(&mut commands, &window_query, image_handle.clone());
    commands.insert_resource(SimulationTexture { image_handle });
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d::default());
}

fn create_grid_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let size = Extent3d {
        width: GRID_WIDTH as u32,
        height: GRID_HEIGHT as u32,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new(
        size,
        TextureDimension::D2,
        vec![0; GRID_WIDTH * GRID_HEIGHT * 4],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    initialize_texture(&mut image);
    images.add(image)
}

fn initialize_texture(image: &mut Image) {
    for pixel in image.data.chunks_exact_mut(4) {
        pixel[0] = 0;   // R
        pixel[1] = 0;   // G
        pixel[2] = 0;   // B
        pixel[3] = 255; // A
    }
}

fn spawn_grid_sprite(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    image_handle: Handle<Image>,
) {
    let window = window_query.single();
    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(
                (GRID_WIDTH as f32) * CELL_SIZE,
                (GRID_HEIGHT as f32) * CELL_SIZE,
            )),
            anchor: Anchor::Custom(Vec2::new(-0.5, 0.0)),
            ..Default::default()
        },
        Transform::from_xyz(-window.width() / 2.0, 0.0, 0.0),
    ));
}