use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod screens;
mod states;

use resources::MixColor;
use screens::{game::*, menu::*};
use states::AppState;

use crate::events::ColorMixedEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Initialize state machine
        .init_state::<AppState>()
        .insert_resource(MixColor(Color::srgb(0.0, 0.0, 0.0)))
        // Register custom message/event queue
        .add_message::<ColorMixedEvent>()
        // Systems for MENU (active only in AppState::Menu)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        // Systems for GAME (active only in AppState::Playing)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            (
                handle_input,
                handle_color_events,
                update_background,
                block_movement_system,
                flash_cooldown_system,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .run();
}
