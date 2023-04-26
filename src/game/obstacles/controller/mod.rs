mod resources;
mod systems;

use crate::game::GameState;
use bevy::prelude::*;

use resources::*;
use systems::*;

pub struct ObstacleControllerPlugin;

impl Plugin for ObstacleControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RockSpawnTimer>()
            .init_resource::<SpaceJunkSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_rock_spawn_timer,
                    update_space_junk_spawn_timer,
                    spawn_rocks_with_timer,
                    spawn_space_junk_with_timer,
                    despawn_rocks_out_of_range,
                    despawn_spacejunk_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
