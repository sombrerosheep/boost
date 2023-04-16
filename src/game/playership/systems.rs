use bevy::prelude::*;

use crate::game::playership::components::*;

const PLAYERSHIP_SPEED: f32 = 325.0;
const PLAYERSHIP_SIZE: f32 = 32.0;

pub fn spawn_player(mut commands: Commands) {
    print!("spawning player");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(400.0, 400.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: PLAYERSHIP_SIZE, y: PLAYERSHIP_SIZE }),
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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>, // todo: input resource to abstract all input kinds
    mut player_query: Query<&mut Transform, With<PlayerShip>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x = -1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x = 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y = 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y = -1.0;
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYERSHIP_SPEED * time.delta_seconds();
    }
}
