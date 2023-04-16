pub mod components;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use systems::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(despawn_player.in_schedule(OnExit(GameState::Playing)));
    }
}
