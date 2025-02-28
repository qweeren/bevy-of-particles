use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::config::WindowConfig;

mod config;
mod systems;
mod materials;
mod grid;
mod utils;
mod registry;
mod plugins;

use plugins::{SimulationPlugin, InputPlugin, UIPlugin};

fn main() {
    let window_config = WindowConfig::new();
    
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Falling Sand Simulator".into(),
                    resolution: (window_config.width, window_config.height).into(),
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
