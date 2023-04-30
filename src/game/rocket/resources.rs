use bevy::prelude::*;

const ROCKET_SPAWN_TIME_SEC: f32 = 4.0;

#[derive(Resource)]
pub struct RocketSpawnTimer {
    pub timer: Timer,
}

impl Default for RocketSpawnTimer {
    fn default() -> RocketSpawnTimer {
        RocketSpawnTimer {
            timer: Timer::from_seconds(ROCKET_SPAWN_TIME_SEC, TimerMode::Repeating),
        }
    }
}
