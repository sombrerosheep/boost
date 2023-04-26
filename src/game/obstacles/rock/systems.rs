use bevy::prelude::*;

use super::components::Rock;

const ROCK_SIZE: f32 = 32.0;

pub fn spawn_rock(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let rock_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
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
        ))
        .id();

    rock_entity
}
