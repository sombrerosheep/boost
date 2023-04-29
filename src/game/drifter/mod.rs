pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct DrifterPlugin;

impl Plugin for DrifterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(update_drifter_position);
    }
}

