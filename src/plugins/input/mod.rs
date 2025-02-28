mod mouse_input;
pub(crate) mod resources;

use bevy::prelude::*;
pub use mouse_input::*;
pub mod input;

// Make the struct public
pub struct InputPlugin;

impl bevy::prelude::Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_click_draw);
    }
}






