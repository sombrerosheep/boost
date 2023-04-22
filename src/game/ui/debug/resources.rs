use bevy::prelude::*;

pub const DEBUG_UI_REDRAW_TIME_SEC: f32 = 1.0;

#[derive(Resource)]
pub struct DebugUIUpdateTimer {
    pub timer: Timer,
}

impl Default for DebugUIUpdateTimer {
    fn default() -> DebugUIUpdateTimer {
        DebugUIUpdateTimer {
            timer: Timer::from_seconds(DEBUG_UI_REDRAW_TIME_SEC, TimerMode::Repeating),
        }
    }
}
