pub mod playership;

use bevy::prelude::*;

pub struct GamePlugin;

use playership::PlayerShipPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerShipPlugin);
    }
}

