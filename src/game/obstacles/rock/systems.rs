use bevy::prelude::*;
use rand::prelude::*;

use super::components::Rock;
use crate::game::camera::game_camera::OBSTACLES_LAYER;
use crate::game::obstacles::components::*;

const SPEED: f32 = 1.0;
const ROCK_SIZE: f32 = 32.0;
pub const MAX_ROTATION: f32 = 1.0;

pub fn spawn_rock(commands: &mut Commands, x: f32, y: f32, rock_rotation: f32) -> Entity {
    let random_x = random::<f32>();
    let random_y = random::<f32>();

    let rand_vel_x = if random_x > 0.5 {
        random_x
    } else {
        random_x * -1.0
    };
    let rand_vel_y = random_y * -1.0;

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
            Drifter {
                velocity: Vec2::new(rand_vel_x, rand_vel_y),
                speed: SPEED,
            },
            Tumbler {
                rotation_speed: rock_rotation,
            },
            Rock {},
        ))
        .id();

    rock_entity
}
