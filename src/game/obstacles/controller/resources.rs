use bevy::prelude::*;

const ROCK_SPAWN_TIME_SEC: f32 = 0.3;
const SPACE_JUNK_SPAWN_TIME_SEC: f32 = 0.35;

#[derive(Resource)]
pub struct RockSpawnTimer {
    pub timer: Timer,
}

impl Default for RockSpawnTimer {
    fn default() -> RockSpawnTimer {
        RockSpawnTimer {
            timer: Timer::from_seconds(ROCK_SPAWN_TIME_SEC, TimerMode::Repeating),
        }
    }
}

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
