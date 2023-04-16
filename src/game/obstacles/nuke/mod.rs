pub mod components;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use systems::*;

pub struct NukePlugin;

impl Plugin for NukePlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_nuke.in_schedule(OnEnter(GameState::Playing)));
    }
}
