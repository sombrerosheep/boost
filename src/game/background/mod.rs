pub mod components;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_background.in_schedule(OnEnter(GameState::Playing)))
            // Systems
            .add_system(update_stars.in_set(OnUpdate(GameState::Playing)))
            // OnExit
            .add_system(despawn_background.in_schedule(OnExit(GameState::Playing)));
    }
}
