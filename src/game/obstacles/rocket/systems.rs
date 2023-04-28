use bevy::prelude::*;

use super::components::Rocket;

const ROCKET_SIZE: Vec2 = Vec2::new(32.0, 64.0);

pub fn spawn_rocket(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            sprite: Sprite {
                custom_size: Some(ROCKET_SIZE),
                color: Color::SILVER,
                ..default()
            },
            ..default()
        },
        Rocket {},
    ));
}
