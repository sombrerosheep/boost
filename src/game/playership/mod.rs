pub mod components;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use crate::AppState;
use systems::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            // OnExit
            .add_system(despawn_player.in_schedule(OnExit(GameState::Playing)))
            // Systems
            .add_systems(
                (player_movement, update_playership_fuel)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
