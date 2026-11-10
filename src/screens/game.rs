use bevy::prelude::*;
use crate::resources::MixColor;
use crate::components::BlockColor;

pub fn setup_game(mut commands: Commands) {
    // Spawn 2D camera for the game view
    commands.spawn(Camera2d);

    // Root layout node for color blocks
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            let colors = [
                Color::srgb(1.0, 0.0, 0.0), // Red
                Color::srgb(0.0, 1.0, 0.0), // Green
                Color::srgb(0.0, 0.0, 1.0), // Blue
            ];

            // Spawn color blocks as UI nodes
            for color in colors {
                parent.spawn((
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    BackgroundColor(color),
                    BlockColor(color),
                ));
            }
        });
}

// Handle global click inputs to mix colors
pub fn block_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mix: ResMut<MixColor>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let current = mix.0.to_srgba();
        // Blend color towards brighter shades on each click
        mix.0 = Color::srgb(
            (current.red + 0.3).min(1.0),
            (current.green + 0.3).min(1.0),
            (current.blue + 0.3).min(1.0),
        );
    }
}

// Update window clear color whenever MixColor resource changes
pub fn update_background(
    mix: Res<MixColor>,
    mut clear: ResMut<ClearColor>,
) {
    if mix.is_changed() {
        clear.0 = mix.0;
    }
}