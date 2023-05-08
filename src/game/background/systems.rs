use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use rand::prelude::*;

use crate::game::game_camera::components::GameCamera;
use crate::game::game_camera::BACKGROUND_LAYER;

const NUM_STARS: u32 = 25;
const STAR_SIZE: f32 = 1.0;
const STAR_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

pub fn spawn_background(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        spawn_star(&mut commands, random_x, random_y);
    }
}

pub fn spawn_star(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let star_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, BACKGROUND_LAYER),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: STAR_SIZE,
                        y: STAR_SIZE,
                    }),
                    color: STAR_COLOR,
                    ..default()
                },
                ..default()
            },
            BackgroundStar { color: STAR_COLOR },
        ))
        .id();

    star_entity
}

pub fn despawn_background(
    mut commands: Commands,
    background_star_query: Query<Entity, With<BackgroundStar>>,
) {
    for background_star in background_star_query.iter() {
        commands.entity(background_star).despawn();
    }
}

pub fn update_stars(
    mut star_query: Query<&mut Transform, (With<BackgroundStar>, Without<Camera2d>)>,
    camera_query: Query<&Transform, (With<GameCamera>, Without<BackgroundStar>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(camera_pos) = camera_query.get_single() {
        let camera_bottom = camera_pos.translation.y - window.height() / 2.0;

        for mut star_position in star_query.iter_mut() {
            if star_position.translation.y < camera_bottom {
                let move_vec: Vec3 = Vec3::new(0.0, window.height(), 0.0);

                star_position.translation += move_vec;
            }
        }
    }
}
