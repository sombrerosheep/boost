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
            .init_resource::<SatelliteSpawnTimer>()
            .init_resource::<RocketSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_rock_spawn_timer,
                    update_space_junk_spawn_timer,
                    update_satellite_spawn_timer,
                    update_rocket_spawn_timer,
                    spawn_rocks_with_timer,
                    spawn_spacejunk_with_timer,
                    spawn_satellites_with_timer,
                    spawn_rocket_with_timer,
                    despawn_rocks_out_of_range,
                    despawn_spacejunk_out_of_range,
                    despawn_satellites_out_of_range,
                    despawn_rockets_out_of_range,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
