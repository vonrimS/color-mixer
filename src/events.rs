use bevy::prelude::*;

/// Event triggered when a player interacts to mix colors
#[derive(Event, Debug, Clone, Copy, Message)]
pub struct ColorMixedEvent;
