use bevy::prelude::*;

pub mod components;
pub mod controller;
pub mod nuke;
pub mod rock;
pub mod rocket;
pub mod satellite;
pub mod spacejunk;
pub mod systems;

use systems::*;

pub struct DrifterPlugin;

impl Plugin for DrifterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(update_drifter_position);
    }
}

pub struct TumblerPlugin;

impl Plugin for TumblerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(update_tumbler_rotation);
    }
}
