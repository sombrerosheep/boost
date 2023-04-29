pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct TumblerPlugin;

impl Plugin for TumblerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(update_tumbler_rotation);
    }
}
