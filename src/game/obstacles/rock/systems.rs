use bevy::prelude::*;

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
    camera_query: Query<(&Transform, &GameCamera)>,
    spawn_timer: Res<RockSpawnTimer>,
) {
    if spawn_timer.timer.finished() {
        if let Ok((camera_pos, game_camera)) = camera_query.get_single() {
            println!("getting viewport");

            let camera_left = camera_pos.translation.x - game_camera.width / 2.0;
            let camera_top = camera_pos.translation.y + game_camera.width / 2.0;

            // spawn a new rock some random place in its y+
            let random_x = random::<f32>() * game_camera.width + camera_left;
            let random_y = random::<f32>() * game_camera.height + camera_top;

            spawn_rock(&mut commands, random_x, random_y);
        }
    }
}

//pub fn despawn_rocks_out_of_range() {}
