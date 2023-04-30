pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use resources::*;
use systems::*;

pub struct RocketSpawnerPlugin;

impl Plugin for RocketSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RocketSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_rocket_spawn_timer,
                    spawn_rocket_with_timer,
                    despawn_rockets_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
