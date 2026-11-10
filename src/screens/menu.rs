use bevy::prelude::*;
use crate::states::AppState;
use crate::components::MenuTag;

pub fn setup_menu(mut commands: Commands) {
    // Spawn 2D camera for the menu UI
    commands.spawn(Camera2d);

    // Root UI node covering the whole screen
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuTag,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Press SPACE to Start Game"),
                TextFont {
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuTag>>,
) {
    // Transition to Playing state when SPACE is pressed
    if keyboard.just_pressed(KeyCode::Space) {
        for entity in &menu_query {
            commands.entity(entity).despawn();
        }
        next_state.set(AppState::Playing);
    }
}