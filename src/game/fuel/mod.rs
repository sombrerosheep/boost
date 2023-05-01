use bevy::prelude::*;

use crate::game::GameState;
use crate::AppState;

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(
                update_fuel
                    .in_set(OnUpdate(GameState::Playing))
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}

#[derive(Component)]
pub struct Fuel {
    pub capacity: f32,
    pub level: f32,
    pub usage: f32,
}

impl Fuel {
    pub fn new(capacity: f32, usage_rate: f32) -> Fuel {
        Fuel {
            capacity,
            level: capacity,
            usage: usage_rate,
        }
    }

    fn step(&mut self, delta_seconds: f32) {
        self.level -= self.usage * delta_seconds;

        if self.level < 0.0 {
            self.level = 0.0;
        }
    }
}

pub fn update_fuel(mut fuel_query: Query<&mut Fuel>, time: Res<Time>) {
    for mut fuel in fuel_query.iter_mut() {
        fuel.step(time.delta_seconds());
    }
}
