use bevy::prelude::*;
use grid::Grid;

mod systems;
mod materials;
mod grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling Sand Simulator".into(),
                resolution: (1000.0, 1000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, systems::setup)
        .insert_resource(Grid::new()) // Add the grid as a resource
        .run();
}
