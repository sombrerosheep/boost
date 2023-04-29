use bevy::prelude::*;

use super::components::Nuke;
use crate::game::camera::game_camera::OBSTACLES_LAYER;

const NUKE_SIZE: f32 = 64.0;

pub fn spawn_nuke(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, OBSTACLES_LAYER),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: NUKE_SIZE,
                    y: NUKE_SIZE,
                }),
                color: Color::YELLOW,
                ..default()
            },
            ..default()
        },
        Nuke {},
    ));
}
