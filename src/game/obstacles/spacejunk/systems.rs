use bevy::prelude::*;

use super::components::SpaceJunk;

const SPACEJUNK_SIZE: f32 = 16.0;

pub fn spawn_spacejunk(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(500.0, 100.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPACEJUNK_SIZE, SPACEJUNK_SIZE)),
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
        SpaceJunk {},
    ));
}
