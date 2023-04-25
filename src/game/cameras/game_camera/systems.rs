use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

const CAMERA_SPEED: f32 = 100.0;

pub fn spawn_game_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        GameCamera {},
    ));
}

pub fn despawn_game_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<GameCamera>>
) {
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn();
    }
}

pub fn move_game_camera(mut game_camera: Query<&mut Transform, With<GameCamera>>, time: Res<Time>) {
    if let Ok(mut transform) = game_camera.get_single_mut() {
        let move_vec: Vec3 = Vec3::new(0.0, CAMERA_SPEED, 0.0);

        transform.translation += move_vec * time.delta_seconds();
    }
}
