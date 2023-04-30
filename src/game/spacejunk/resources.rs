use bevy::prelude::*;

const SPACE_JUNK_SPAWN_TIME_SEC: f32 = 0.45;

#[derive(Resource)]
pub struct SpaceJunkSpawnTimer {
    pub timer: Timer,
}

impl Default for SpaceJunkSpawnTimer {
    fn default() -> SpaceJunkSpawnTimer {
        SpaceJunkSpawnTimer {
            timer: Timer::from_seconds(SPACE_JUNK_SPAWN_TIME_SEC, TimerMode::Repeating),
        }
    }
}
