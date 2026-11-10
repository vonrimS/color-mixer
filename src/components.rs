use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct BlockColor(pub Color);

/// Marker component for menu entities to easily clean them up on state transition
#[derive(Component)]
pub struct MenuTag;

#[derive(Component, Debug)]
pub struct Velocity(pub f32);
