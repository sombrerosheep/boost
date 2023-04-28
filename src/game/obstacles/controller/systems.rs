use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;

use super::resources::*;
use crate::game::cameras::game_camera::components::GameCamera;
use crate::game::obstacles::rock::{components::*, systems::*};
use crate::game::obstacles::rocket::{components::*, systems::*};
use crate::game::obstacles::satellite::{components::*, systems::*};
use crate::game::obstacles::spacejunk::{components::*, systems::*};

pub fn update_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}

pub fn update_space_junk_spawn_timer(
    mut junk_spawn_timer: ResMut<SpaceJunkSpawnTimer>,
    time: Res<Time>,
) {
    junk_spawn_timer.timer.tick(time.delta());
}

pub fn update_satellite_spawn_timer(
    mut satellite_spawn_timer: ResMut<SatelliteSpawnTimer>,
    time: Res<Time>,
) {
    satellite_spawn_timer.timer.tick(time.delta());
}

pub fn update_rocket_spawn_timer(
    mut rocket_spawn_timer: ResMut<RocketSpawnTimer>,
    time: Res<Time>,
) {
    rocket_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_rocks_with_timer(
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

pub fn spawn_satellites_with_timer(
    mut commands: Commands,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    spawn_timer: Res<SatelliteSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if spawn_timer.timer.finished() {
        if let Ok(camera_pos) = camera_query.get_single() {
            let camera_left = camera_pos.translation.x - window.width() / 2.0;
            let camera_top = camera_pos.translation.y + window.width() / 2.0;

            let random_x = random::<f32>() * window.width() + camera_left;
            let random_y = random::<f32>() * window.height() + camera_top;

            spawn_satellite(&mut commands, random_x, random_y);
        }
    }
}

pub fn spawn_rocket_with_timer(
    mut commands: Commands,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    spawn_timer: Res<RocketSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if spawn_timer.timer.finished() {
        if let Ok(camera_pos) = camera_query.get_single() {
            let camera_left = camera_pos.translation.x - window.width() / 2.0;
            let camera_top = camera_pos.translation.y + window.width() / 2.0;

            let random_x = random::<f32>() * window.width() + camera_left;
            let random_y = random::<f32>() * window.height() + camera_top;

            spawn_rocket(&mut commands, random_x, random_y);
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
                commands.entity(rock_entity).despawn()
            }
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

pub fn despawn_satellites_out_of_range(
    mut commands: Commands,
    satellite_query: Query<(Entity, &Transform), With<Satellite>>,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(camera_pos) = camera_query.get_single() {
        let bottom_buffer = 50.0;
        let bottom_y = camera_pos.translation.y - window.height() / 2.0 - bottom_buffer;

        for (entity, transform) in satellite_query.iter() {
            if transform.translation.y < bottom_y {
                commands.entity(entity).despawn()
            }
        }
    }
}

pub fn despawn_rockets_out_of_range(
    mut commands: Commands,
    rocket_query: Query<(Entity, &Transform), With<Rocket>>,
    camera_query: Query<&Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(camera_pos) = camera_query.get_single() {
        let bottom_buffer = 50.0;
        let bottom_y = camera_pos.translation.y - window.height() / 2.0 - bottom_buffer;

        for (entity, transform) in rocket_query.iter() {
            if transform.translation.y < bottom_y {
                commands.entity(entity).despawn()
            }
        }
    }
}
