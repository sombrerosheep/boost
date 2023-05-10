pub mod components;
mod layout;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use systems::*;

use self::layout::*;

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_gamehud_ui.in_schedule(OnEnter(GameState::Playing)))
            // Systems
            .add_system(update_fuel_level_gauge.in_set(OnUpdate(GameState::Playing)))
            // OnExit
            .add_system(despawn_gamehud_ui.in_schedule(OnExit(GameState::Playing)));
    }
}
