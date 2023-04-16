pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(spawn_rocket.in_schedule(OnEnter(GameState::Playing)));
    }
}
