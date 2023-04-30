use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::SpaceJunk;
use super::resources::*;
use crate::game::camera::game_camera::components::*;
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

pub fn update_space_junk_spawn_timer(
    mut junk_spawn_timer: ResMut<SpaceJunkSpawnTimer>,
    time: Res<Time>,
) {
    junk_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_spacejunk_with_timer(
    mut commands: Commands,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    spawn_timer: Res<SpaceJunkSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if spawn_timer.timer.finished() {
        if let Ok(camera_pos) = camera_query.get_single() {
            let camera_left = camera_pos.translation.x - window.width() / 2.0;
            let camera_top = camera_pos.translation.y + window.width() / 2.0;

            let random_x = random::<f32>() * window.width() + camera_left;
            let random_y = random::<f32>() * window.height() + camera_top;

            spawn_spacejunk(&mut commands, random_x, random_y);
        }
    }
}

pub fn despawn_spacejunk_out_of_range(
    mut commands: Commands,
    spacejunk_query: Query<(Entity, &Transform), With<SpaceJunk>>,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(camera_pos) = camera_query.get_single() {
        let bottom_buffer = 50.0;
        let bottom_y = camera_pos.translation.y - window.height() / 2.0 - bottom_buffer;

        for (entity, transform) in spacejunk_query.iter() {
            if transform.translation.y < bottom_y {
                commands.entity(entity).despawn()
            }
        }
    }
}
