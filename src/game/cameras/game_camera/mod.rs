pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup
            .add_system(spawn_game_camera.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_game_camera.in_set(OnUpdate(GameState::Playing)));
    }
}
