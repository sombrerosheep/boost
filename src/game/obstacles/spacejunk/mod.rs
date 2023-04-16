pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct SpaceJunkPlugin;

impl Plugin for SpaceJunkPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_spacejunk.in_schedule(OnEnter(GameState::Playing)));
    }
}
