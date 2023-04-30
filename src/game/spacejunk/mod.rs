pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::game::GameState;
use resources::*;
use systems::*;

pub struct SpaceJunkSpawnerPlugin;

impl Plugin for SpaceJunkSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<SpaceJunkSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_space_junk_spawn_timer,
                    spawn_spacejunk_with_timer,
                    despawn_spacejunk_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
