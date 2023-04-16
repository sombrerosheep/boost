use bevy::prelude::*;

mod game;
mod systems;

use game::GamePlugin;
use systems::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
}

fn main() {
    println!("Hello, world!");

    App::new()
        // Built-ins
        .add_plugins(DefaultPlugins)
        // State
        .add_state::<AppState>()
        // Startup
        .add_startup_system(spawn_main_camera)
        // Plugins
        .add_plugin(GamePlugin)
        // Systems
        .add_system(exit_game_on_esc)
        .run();
}
