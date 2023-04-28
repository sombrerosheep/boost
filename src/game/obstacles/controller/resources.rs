use bevy::prelude::*;

const ROCK_SPAWN_TIME_SEC: f32 = 0.55;
const SPACE_JUNK_SPAWN_TIME_SEC: f32 = 0.45;
const SATELLITE_SPAWN_TIME_SEC: f32 = 1.1;
const ROCKET_SPAWN_TIME_SEC: f32 = 4.0;
const NUKE_SPAWN_TIME_SEC: f32 = 10.0;

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
