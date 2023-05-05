pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(apply_velocity_to_transform);
    }
}
