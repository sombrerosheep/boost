use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Rock;
use super::resources::RockSpawnTimer;

use crate::game::cameras::game_camera::components::GameCamera;
use rand::prelude::*;

const ROCK_SIZE: f32 = 32.0;

pub fn spawn_rock(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let rock_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ROCK_SIZE,
                        y: ROCK_SIZE,
                    }),
                    color: Color::BEIGE,
                    ..default()
                },
                ..default()
            },
            Rock {},
        ))
        .id();

    rock_entity
}

pub fn update_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_rocks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    spawn_timer: Res<RockSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if spawn_timer.timer.finished() {
        if let Ok(camera_pos) = camera_query.get_single() {
            let camera_left = camera_pos.translation.x - window.width() / 2.0;
            let camera_top = camera_pos.translation.y + window.width() / 2.0;

            // spawn a new rock some random place in its y+
            let random_x = random::<f32>() * window.width() + camera_left;
            let random_y = random::<f32>() * window.height() + camera_top;

            // println!("spawning rock at: {}x{}", random_x, random_y);
            spawn_rock(&mut commands, random_x, random_y);
        }
    }
}

pub fn despawn_rocks_out_of_range(
    mut commands: Commands,
    rock_query: Query<(Entity, &Transform), With<Rock>>,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(camera_pos) = camera_query.get_single() {
        let bottom_buffer = 50.0;
        let bottom_y = camera_pos.translation.y - window.height() / 2.0 - bottom_buffer;

        for (rock_entity, transform) in rock_query.iter() {
            if transform.translation.y < bottom_y {
                // println!("despawning rock at {}x{} ({})",
                //         transform.translation.x, transform.translation.y, bottom_y);
                commands.entity(rock_entity).despawn()
            }
        }
    }
}
