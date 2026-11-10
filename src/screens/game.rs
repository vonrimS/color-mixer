use crate::components::{BlockColor, Velocity};
use crate::events::ColorMixedEvent;
use crate::resources::MixColor;
use bevy::prelude::*;

pub fn setup_game(mut commands: Commands) {
    // Spawn 2D camera for the game view
    commands.spawn(Camera2d);

    let blocks = [
        (Color::srgb(1.0, 0.0, 0.0), 120.0),
        (Color::srgb(0.0, 1.0, 0.0), 0.0),
        (Color::srgb(0.0, 0.0, 1.0), -80.0),
    ];

    let start_x = 300.0;
    let block_size = 100.0;
    let spacing = 20.0; // отступ между блоками

    for (i, (color, initial_velocity)) in blocks.into_iter().enumerate() {
        let x_pos = start_x + (i as f32) * (block_size + spacing);

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x_pos),
                top: Val::Px(300.0), // По центру экрана по вертикали
                width: Val::Px(block_size),
                height: Val::Px(block_size),
                ..default()
            },
            BackgroundColor(color),
            BlockColor(color),
            Velocity(initial_velocity),
        ));
    }
}

// System 1: Captures mouse input and emits a color mixing message
// completely decoupled from the actual state mutation logic
pub fn handle_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut messages: MessageWriter<ColorMixedEvent>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        messages.write(ColorMixedEvent);
    }
}

// System 2: Listens for color mixing messages and updates the global MixColor resource reactively
pub fn handle_color_events(
    mut messages: MessageReader<ColorMixedEvent>,
    mut mix: ResMut<MixColor>,
) {
    for _event in messages.read() {
        let current = mix.0.to_srgba();
        mix.0 = Color::srgb(
            (current.red + 0.3).min(1.0),
            (current.green + 0.3).min(1.0),
            (current.blue + 0.3).min(1.0),
        );
        info!("ColorMixedEvent received! Updating mix color.");
    }
}

// Update window clear color whenever MixColor resource changes
pub fn update_background(mix: Res<MixColor>, mut clear: ResMut<ClearColor>) {
    if mix.is_changed() {
        clear.0 = mix.0;
    }
}

pub fn block_movement_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Node, &mut Velocity)>,
) {
    let dt = time.delta_secs();
    let block_width = 100.0;
    let min_x = 50.0;
    let max_x = 950.0;

    let mut data: Vec<(Entity, f32, f32)> = query
        .iter()
        .map(|(entity, node, vel)| {
            let left = match node.left {
                Val::Px(val) => val,
                _ => 100.0,
            };
            (entity, left, vel.0)
        })
        .collect();

    data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for item in data.iter_mut() {
        item.1 += item.2 * dt;

        if item.1 <= min_x {
            item.1 = min_x;
            item.2 = -item.2;
        } else if item.1 >= max_x - block_width {
            item.1 = max_x - block_width;
            item.2 = -item.2;
        }
    }

    for i in 0..data.len().saturating_sub(1) {
        let right_side_a = data[i].1 + block_width;
        let left_side_b = data[i + 1].1;

        if right_side_a > left_side_b {
            let push_back = (right_side_a - left_side_b) / 2.0;
            data[i].1 -= push_back;
            data[i + 1].1 += push_back;

            let temp_vel = data[i].2;
            data[i].2 = data[i + 1].2;
            data[i + 1].2 = temp_vel;
        }
    }

    for (entity, new_left, new_vel) in data {
        if let Ok((_, mut node, mut vel)) = query.get_mut(entity) {
            node.left = Val::Px(new_left);
            vel.0 = new_vel;
        }
    }
}
