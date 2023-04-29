pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub const BACKGROUND_LAYER: f32 = -1.0;
pub const PLAYER_LAYER: f32 = -0.5;
pub const OBSTACLES_LAYER: f32 = 0.0;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup
            .add_system(spawn_game_camera.in_schedule(OnEnter(GameState::Playing)))
            // Systems
            .add_system(move_game_camera.in_set(OnUpdate(GameState::Playing)))
            // OnExit
            .add_system(despawn_game_camera.in_schedule(OnExit(GameState::Playing)));
    }
}
