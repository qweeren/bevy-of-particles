use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Drawing(pub bool);

// Resource to store the last mouse grid position
#[derive(Resource, Default)]
pub struct LastMouseGridPos(pub Option<(usize, usize)>);