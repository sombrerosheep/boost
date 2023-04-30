use bevy::prelude::*;

const SATELLITE_SPAWN_TIME_SEC: f32 = 1.1;

#[derive(Resource)]
pub struct SatelliteSpawnTimer {
    pub timer: Timer,
}

impl Default for SatelliteSpawnTimer {
    fn default() -> SatelliteSpawnTimer {
        SatelliteSpawnTimer {
            timer: Timer::from_seconds(SATELLITE_SPAWN_TIME_SEC, TimerMode::Repeating),
        }
    }
}
