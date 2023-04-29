use bevy::prelude::*;

use super::components::Satellite;

const SATELLITE_SIZE: Vec2 = Vec2::new(64.0, 32.0);
use crate::game::camera::game_camera::OBSTACLES_LAYER;

pub fn spawn_satellite(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, OBSTACLES_LAYER),
                sprite: Sprite {
                    custom_size: Some(SATELLITE_SIZE),
                    color: Color::ALICE_BLUE,
                    ..default()
                },
                ..default()
            },
            Satellite {},
        ))
        .id();

    entity
}
