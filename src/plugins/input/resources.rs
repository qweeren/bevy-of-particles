use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BrushSize(pub u8);

#[derive(Resource, Default)]
pub struct SelectedMaterial(pub u8);
