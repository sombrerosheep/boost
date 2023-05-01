mod background;
pub mod camera;
pub mod drifter;
pub mod fuel;
pub mod nuke;
pub mod playership;
pub mod rock;
pub mod rocket;
pub mod satellite;
pub mod spacejunk;
pub mod tumbler;
mod ui;

use background::BackgroundPlugin;
use bevy::prelude::*;
use camera::game_camera::GameCameraPlugin;
use playership::PlayerShipPlugin;

use drifter::DrifterPlugin;
use tumbler::TumblerPlugin;

use nuke::NukeSpawnerPlugin;
use rock::RockSpawnerPlugin;
use rocket::RocketSpawnerPlugin;
use satellite::SatelliteSpawnerPlugin;
use spacejunk::SpaceJunkSpawnerPlugin;

use fuel::FuelPlugin;

use ui::debug::DebugUIPlugin;

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
            .add_plugin(DrifterPlugin)
            .add_plugin(TumblerPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(NukeSpawnerPlugin)
            .add_plugin(RockSpawnerPlugin)
            .add_plugin(RocketSpawnerPlugin)
            .add_plugin(SatelliteSpawnerPlugin)
            .add_plugin(SpaceJunkSpawnerPlugin)
            .add_plugin(FuelPlugin)
            // UI
            .add_plugin(DebugUIPlugin);
    }
}
