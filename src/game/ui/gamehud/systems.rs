use super::components::*;
use bevy::prelude::*;

use super::layout::FUEL_GAUGE_HEIGHT;
use crate::game::fuel::Fuel;
use crate::game::playership::components::*;

pub fn update_fuel_level_gauge(
    mut ui_query: Query<&mut Style, With<FuelLevelGauge>>,
    player_fuel: Query<&Fuel, With<PlayerShip>>,
) {
    let fuel = player_fuel.get_single().unwrap();

    for mut node in ui_query.iter_mut() {
        node.size.height = Val::Px((fuel.level / fuel.capacity) * FUEL_GAUGE_HEIGHT);
    }
}
