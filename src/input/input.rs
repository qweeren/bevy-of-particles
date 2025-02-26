use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Drawing(pub bool);

// Resource to store the last mouse grid position
#[derive(Resource, Default)]
pub struct LastMouseGridPos {
    pub x: Option<usize>,
    pub y: Option<usize>,
}

#[derive(Resource)]
pub struct BrushSize(pub usize); // Radius in cells

#[derive(Resource)]
pub struct SelectedMaterial(pub u8);