use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 1. Создаем 2D-камеру
    commands.spawn(Camera2d);

    // 2. Спавним синий квадрат
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 0.7, 1.0),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    });
}