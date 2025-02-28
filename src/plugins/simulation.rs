use bevy::prelude::*;
use crate::grid::Grid;
use crate::systems::{setup, update_grid, render_grid};

use super::input::input::{Drawing, LastMouseGridPos};
use super::input::resources::{BrushSize, SelectedMaterial};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Grid::new())
            .insert_resource(SelectedMaterial(1))
            .insert_resource(BrushSize(3))
            .insert_resource(LastMouseGridPos::default())
            .insert_resource(Drawing::default())
            .add_systems(Startup, (
                setup
            ))
            .add_systems(Update, (
                update_grid,
                render_grid,
            ));
    }
}
