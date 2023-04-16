pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(spawn_satellite.in_schedule(OnEnter(GameState::Playing)));
    }
}
