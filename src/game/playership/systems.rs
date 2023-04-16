use bevy::prelude::*;

use crate::game::playership::components::*;

pub fn spawn_player(mut commands: Commands) {
    print!("spawning player");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(400.0, 400.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 16.0, y: 16.0 }),
                color: Color::rgb(0.4, 0.7, 0.3),
                ..default()
            },
            ..default()
        },
        PlayerShip {},
    ));
}

pub fn despawn_player(mut commands: Commands, playership_query: Query<Entity, With<PlayerShip>>) {
    if let Ok(player_ship) = playership_query.get_single() {
        commands.entity(player_ship).despawn();
    }
}
