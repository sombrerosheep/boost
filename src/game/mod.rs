pub mod obstacles;
pub mod playership;
mod systems;
mod cameras;

use bevy::prelude::*;
use obstacles::nuke::NukePlugin;
use obstacles::rock::RockPlugin;
use obstacles::rocket::RocketPlugin;
use obstacles::satellite::SatellitePlugin;
use obstacles::spacejunk::SpaceJunkPlugin;
use playership::PlayerShipPlugin;
use cameras::game_camera::GameCameraPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    Playing,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // State
            .add_state::<GameState>()
            // Plugins
            .add_plugin(GameCameraPlugin)
            .add_plugin(PlayerShipPlugin)
            .add_plugin(NukePlugin)
            .add_plugin(RockPlugin)
            .add_plugin(RocketPlugin)
            .add_plugin(SatellitePlugin)
            .add_plugin(SpaceJunkPlugin);
            // Systems
    }
}
