mod setup;
mod update;
mod render;

pub use setup::{setup, SimulationTexture};
pub use update::update_grid;
pub use render::render_grid;