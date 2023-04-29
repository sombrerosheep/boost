use bevy::prelude::*;

use super::components::Rock;
use crate::game::camera::game_camera::OBSTACLES_LAYER;

const ROCK_SIZE: f32 = 32.0;
pub const MAX_ROTATION: f32 = 1.0;

pub fn spawn_rock(commands: &mut Commands, x: f32, y: f32, rock_rotation: f32) -> Entity {
    let rock_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, OBSTACLES_LAYER),
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
            Rock {
                rotation: rock_rotation,
            },
        ))
        .id();

    rock_entity
}

pub fn update_rock_rotation(
    mut rock_query: Query<(&mut Transform, &Rock), With<Rock>>,
    time: Res<Time>,
) {
    for (mut transform, rock) in rock_query.iter_mut() {
        transform.rotate_z(rock.rotation * time.delta_seconds())
    }
}
