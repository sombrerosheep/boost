use bevy::prelude::*;
use rand::prelude::*;

use super::components::Rock;
use crate::game::camera::game_camera::OBSTACLES_LAYER;
use crate::game::drifter::components::*;
use crate::game::tumbler::components::*;

const SPEED: f32 = 1.0;
const ROCK_SIZE: f32 = 32.0;
pub const MAX_ROTATION: f32 = 1.0;

pub fn spawn_rock(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let rotation_rand = random::<f32>();
    let rotation_dir = if rotation_rand < 0.5 { 1.0 } else { -1.0 };
    let rock_rotation = rotation_rand * MAX_ROTATION * rotation_dir;

    let random_vx = random::<f32>();
    let random_vy = random::<f32>();
    let rock_vx = if random_vx > 0.5 {
        random_vx
    } else {
        random_vx * -1.0
    };
    let rock_vy = random_vy * -1.0;

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
                velocity: Vec2::new(rock_vx, rock_vy),
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
