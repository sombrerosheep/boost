pub mod components;
mod resources;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;
use resources::RockSpawnTimer;
use systems::*;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RockSpawnTimer>()
            // OnEnter
            .add_systems((
                update_rock_spawn_timer,
                spawn_rocks,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}
