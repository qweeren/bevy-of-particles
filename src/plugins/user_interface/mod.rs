pub mod ui;

use bevy::prelude::*;
use ui::ui_system;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}
