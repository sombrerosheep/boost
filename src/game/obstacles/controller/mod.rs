mod resources;
mod systems;

use bevy::prelude::*;
use crate::game::GameState;

use resources::*;
use systems::*;

pub struct ObstacleControllerPlugin;

impl Plugin for ObstacleControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RockSpawnTimer>()
            // Systems
            .add_systems(
                (
                    update_rock_spawn_timer,
                    spawn_rocks_with_timer,
                    despawn_rocks_out_of_range
                ).in_set(OnUpdate(GameState::Playing))
            );
    }
}
