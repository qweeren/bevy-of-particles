use bevy::{prelude::*, render::texture::DefaultImageSampler};
use grid::Grid;
use input::input::{BrushSize, LastMouseGridPos, SelectedMaterial};
use crate::input::mouse_click_draw::mouse_click_draw;
use crate::input::input::Drawing;

mod systems;
mod materials;
mod grid;
mod utils;
mod input;
mod user_interface;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling Sand Simulator".into(),
                resolution: (1000.0, 1000.0).into(),
                ..default()
            }),
            ..default()
        }).set(bevy::render::texture::ImagePlugin::default_nearest()))
        .insert_resource(Grid::new())
        .insert_resource(SelectedMaterial(1)) // Default to sand
        .insert_resource(BrushSize(3)) // Default brush size
        .insert_resource(LastMouseGridPos::default())
        .insert_resource(Drawing::default())
        .add_systems(Startup, systems::setup)
        .add_systems(Startup, systems::spawn_test_materials.after(systems::setup))
        .add_systems(
            Update,
            (
                systems::update_grid,
                systems::render_grid,
                mouse_click_draw,
            ),
        )
        .run();
}