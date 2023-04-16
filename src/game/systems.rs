use bevy::prelude::*;

const CAMERA_SPEED: f32 = 100.0;

pub fn move_camera(mut game_camera: Query<&mut Transform, With<Camera2d>>, time: Res<Time>) {
    if let Ok(mut transform) = game_camera.get_single_mut() {
        let move_vec: Vec3 = Vec3::new(0.0, CAMERA_SPEED, 0.0);

        transform.translation += move_vec * time.delta_seconds();
    }
}
