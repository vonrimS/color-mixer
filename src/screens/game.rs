use crate::components::{BlockColor, FlashTimer, Velocity};
use crate::events::ColorMixedEvent;
use crate::resources::MixColor;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Spawns the initial game scene, including the 2D camera and a row of interactive blocks
/// with varying initial velocities to simulate a Newton's Cradle style movement.
pub fn setup_game(mut commands: Commands) {
    // Spawn 2D camera for the game view
    commands.spawn(Camera2d);

    let blocks = [
        (Color::srgb(1.0, 0.0, 0.0), 120.0), // Left block (fast)
        (Color::srgb(0.0, 1.0, 0.0), 0.0),   // Middle block (stationary/slow)
        (Color::srgb(0.0, 0.0, 1.0), -80.0), // Right block (moving towards center)
    ];

    let start_x = 300.0;
    let block_size = 100.0;
    let spacing = 20.0; // Spacing between adjacent blocks

    for (i, (color, initial_velocity)) in blocks.into_iter().enumerate() {
        let x_pos = start_x + (i as f32) * (block_size + spacing);

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x_pos),
                top: Val::Px(300.0), // Centered vertically on the screen
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

/// System responsible for block movement, window boundary constraints,
/// elastic collisions between blocks, and triggering impact flashes.
pub fn block_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Node, &mut Velocity, &mut BackgroundColor)>,
) {
    // Dynamically retrieve the window width to bound movement accurately
    let screen_width = window_query.single().map(|w| w.width()).unwrap_or(1280.0);

    let dt = time.delta_secs();
    let block_width = 100.0;
    let min_x = 0.0;
    let max_x = screen_width;

    // Extract current positions and velocities into a mutable vector for sorting and collision handling
    let mut data: Vec<(Entity, f32, f32)> = query
        .iter()
        .map(|(entity, node, vel, _)| {
            let left = match node.left {
                Val::Px(val) => val,
                _ => 100.0,
            };
            (entity, left, vel.0)
        })
        .collect();

    // Sort blocks from left to right along the X-axis for correct neighbor collision detection
    data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut collided_entities = Vec::new();

    // 1. Move blocks and check window boundary collisions
    for item in data.iter_mut() {
        item.1 += item.2 * dt;

        if item.1 <= min_x {
            item.1 = min_x;
            item.2 = -item.2;
            collided_entities.push(item.0);
        } else if item.1 >= max_x - block_width {
            item.1 = max_x - block_width;
            item.2 = -item.2;
            collided_entities.push(item.0);
        }
    }

    // 2. Check elastic collisions between adjacent blocks
    for i in 0..data.len().saturating_sub(1) {
        let right_side_a = data[i].1 + block_width;
        let left_side_b = data[i + 1].1;

        if right_side_a > left_side_b {
            // Push overlapping blocks apart to prevent sticking
            let push_back = (right_side_a - left_side_b) / 2.0;
            data[i].1 -= push_back;
            data[i + 1].1 += push_back;

            // Swap velocities (elastic collision momentum transfer)
            let temp_vel = data[i].2;
            data[i].2 = data[i + 1].2;
            data[i + 1].2 = temp_vel;

            collided_entities.push(data[i].0);
            collided_entities.push(data[i + 1].0);
        }
    }

    // 3. Apply updated positions, velocities, and trigger flash effect on impact
    for (entity, new_left, new_vel) in data {
        if let Ok((_, mut node, mut vel, mut bg_color)) = query.get_mut(entity) {
            node.left = Val::Px(new_left);
            vel.0 = new_vel;

            if collided_entities.contains(&entity) {
                bg_color.0 = Color::WHITE;
                commands
                    .entity(entity)
                    .insert(FlashTimer(Timer::from_seconds(0.3, TimerMode::Once)));
            }
        }
    }
}

/// Ticks active flash timers and smoothly fades block colors back from white to their original colors.
pub fn flash_cooldown_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackgroundColor, &BlockColor, &mut FlashTimer)>,
) {
    for (entity, mut bg_color, original_color, mut flash_timer) in &mut query {
        flash_timer.0.tick(time.delta());

        if flash_timer.0.is_finished() {
            bg_color.0 = original_color.0;
            commands.entity(entity).remove::<FlashTimer>();
        } else {
            // Calculate progress fraction from 0.0 to 1.0 and invert it for fade-out
            let progress = flash_timer.0.fraction();
            let t = 1.0 - progress;

            let white = Srgba::WHITE;
            let orig = original_color.0.to_srgba();

            // Interpolate color smoothly from white back to original block color
            bg_color.0 = Color::from(Srgba::new(
                orig.red + (white.red - orig.red) * t,
                orig.green + (white.green - orig.green) * t,
                orig.blue + (white.blue - orig.blue) * t,
                1.0,
            ));
        }
    }
}
