use bevy::prelude::*;

use crate::game::camera::game_camera::PLAYER_LAYER;
use crate::game::fuel::Fuel;
use crate::game::playership::components::*;

const PLAYERSHIP_SPEED: f32 = 325.0;
const PLAYERSHIP_SIZE: f32 = 32.0;
const PLAYERSHIP_FUEL_CAP: f32 = 10.0;
const PLAYERSHIP_FUEL_RATE: f32 = 0.25;
const PLAYERSHIP_FUEL_BOOST_RATE: f32 = 0.75;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(400.0, 400.0, PLAYER_LAYER),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: PLAYERSHIP_SIZE,
                    y: PLAYERSHIP_SIZE,
                }),
                color: Color::rgb(0.4, 0.7, 0.3),
                ..default()
            },
            ..default()
        },
        Fuel::new(PLAYERSHIP_FUEL_CAP, PLAYERSHIP_FUEL_RATE),
        PlayerShip {},
    ));
}

pub fn despawn_player(mut commands: Commands, playership_query: Query<Entity, With<PlayerShip>>) {
    if let Ok(player_ship) = playership_query.get_single() {
        commands.entity(player_ship).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerShip>>,
    fuel_query: Query<&Fuel, With<PlayerShip>>,
    time: Res<Time>,
) {
    if let Ok(fuel) = fuel_query.get_single() {
        if fuel.level == 0.0 {
            println!("Playership is out of fuel")
        }
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
}

pub fn update_playership_fuel(
    mut fuel_query: Query<&mut Fuel, With<PlayerShip>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut fuel) = fuel_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::LShift) {
            fuel.usage = PLAYERSHIP_FUEL_BOOST_RATE;
        } else {
            fuel.usage = PLAYERSHIP_FUEL_RATE;
        }
    }
}
