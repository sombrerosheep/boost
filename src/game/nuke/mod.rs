pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use resources::*;
use systems::*;

pub struct NukeSpawnerPlugin;

impl Plugin for NukeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<NukeSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_nuke_spawn_timer,
                    spawn_nuke_with_timer,
                    despawn_nukes_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
