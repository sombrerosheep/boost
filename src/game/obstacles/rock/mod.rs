pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_rock.in_schedule(OnEnter(GameState::Playing)));
    }
}
