use bevy::prelude::*;

mod states;
mod components;
mod resources;
mod screens;

use states::AppState;
use resources::MixColor;
use screens::{menu::*, game::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        // Initialize state machine
        .init_state::<AppState>()
        .insert_resource(MixColor(Color::srgb(0.0, 0.0, 0.0)))
        
        // Systems for MENU (active only in AppState::Menu)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        
        // Systems for GAME (active only in AppState::Playing)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            (block_click, update_background).run_if(in_state(AppState::Playing)),
        )
        .run();
}