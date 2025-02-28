use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod systems;
mod materials;
mod grid;
mod utils;
mod registry;
mod plugins;

use plugins::{SimulationPlugin, InputPlugin, UIPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Falling Sand Simulator".into(),
                    resolution: (1000.0 + 230.0, 1000.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(bevy::render::texture::ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins((
            SimulationPlugin,
            InputPlugin,
            UIPlugin,
        ))
        .run();
}
