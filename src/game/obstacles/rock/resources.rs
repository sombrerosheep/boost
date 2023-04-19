use bevy::prelude::*;

pub const ROCK_SPAWN_TIME_SEC: f32 = 0.5;

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
