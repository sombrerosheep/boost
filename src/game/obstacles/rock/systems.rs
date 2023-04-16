use bevy::prelude::*;

use super::components::Rock;

const ROCK_SIZE: f32 = 32.0;

pub fn spawn_rock(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(200.0, 100.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ROCK_SIZE,
                    y: ROCK_SIZE,
                }),
                color: Color::BEIGE,
                ..default()
            },
            ..default()
        },
        Rock {},
    ));
}
