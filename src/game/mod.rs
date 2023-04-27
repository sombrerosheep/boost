mod background;
pub mod cameras;
pub mod obstacles;
pub mod playership;
mod systems;
mod ui;

use bevy::prelude::*;
use cameras::game_camera::GameCameraPlugin;
use obstacles::controller::ObstacleControllerPlugin;
use obstacles::nuke::NukePlugin;
use obstacles::rocket::RocketPlugin;
use playership::PlayerShipPlugin;

use background::BackgroundPlugin;

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
            .add_plugin(ObstacleControllerPlugin)
            .add_plugin(NukePlugin)
            .add_plugin(RocketPlugin)
            .add_plugin(BackgroundPlugin)
            // UI
            .add_plugin(DebugUIPlugin);
    }
}
