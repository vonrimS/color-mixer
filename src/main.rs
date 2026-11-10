use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource)]
struct MixColor(Color);

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
struct BlockColor(Color);

/// Marker component for menu entities to easily clean them up on state transition
#[derive(Component)]
struct MenuTag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Initialize state machine
        .init_state::<AppState>()
        .insert_resource(MixColor(Color::srgb(0.0, 0.0, 0.0)))
        
        // Systems for MENU (works only in AppState::Menu)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))

        // Systems for GAME (works only in AppState::Playing)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            (block_click, update_background).run_if(in_state(AppState::Playing)),
        )
        .run();
}


// --- MENU STATE ---

fn setup_menu(mut commands: Commands) {
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

fn menu_input(
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


// --- GAMEPLAY STATE ---

fn setup_game(mut commands: Commands) {
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
                Color::srgb(1.0, 0.0, 0.0),  // Red
                Color::srgb(0.0, 1.0, 0.0),  // Green
                Color::srgb(0.0, 0.0, 1.0),  // Blue
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
fn block_click(mouse_button: Res<ButtonInput<MouseButton>>, mut mix: ResMut<MixColor>) {
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
fn update_background(mix: Res<MixColor>, mut clear: ResMut<ClearColor>) {
    if mix.is_changed() {
        clear.0 = mix.0;
    }
}
