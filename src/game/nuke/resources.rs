use bevy::prelude::*;

const NUKE_SPAWN_TIME_SEC: f32 = 10.0;

#[derive(Resource)]
pub struct NukeSpawnTimer {
    pub timer: Timer,
}

impl Default for NukeSpawnTimer {
    fn default() -> NukeSpawnTimer {
        NukeSpawnTimer {
            timer: Timer::from_seconds(NUKE_SPAWN_TIME_SEC, TimerMode::Repeating),
        }
    }
}
