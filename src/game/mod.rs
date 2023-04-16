pub mod playership;

use bevy::prelude::*;
use playership::PlayerShipPlugin;

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
        print!("building game plugin");
        app.add_state::<GameState>().add_plugin(PlayerShipPlugin);
    }
}
