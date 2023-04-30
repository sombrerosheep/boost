pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use resources::*;
use systems::*;

pub struct SatelliteSpawnerPlugin;

impl Plugin for SatelliteSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<SatelliteSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_satellite_spawn_timer,
                    spawn_satellites_with_timer,
                    despawn_satellites_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
