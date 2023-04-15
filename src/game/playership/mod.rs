pub mod components;
mod systems;

use bevy::prelude::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, _app: &mut App) {}
}
