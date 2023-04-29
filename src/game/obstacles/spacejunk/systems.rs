use bevy::prelude::*;

use super::components::SpaceJunk;
use crate::game::camera::game_camera::OBSTACLES_LAYER;

const SPACEJUNK_SIZE: f32 = 16.0;

pub fn spawn_spacejunk(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, OBSTACLES_LAYER),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SPACEJUNK_SIZE, SPACEJUNK_SIZE)),
                    color: Color::GREEN,
                    ..default()
                },
                ..default()
            },
            SpaceJunk {},
        ))
        .id();

    entity
}
