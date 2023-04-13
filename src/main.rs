use bevy::prelude::*;

mod game;

use game::GamePlugin;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
