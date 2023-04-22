pub mod components;
mod layout;
mod systems;
mod resources;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use crate::AppState;
use layout::*;
use systems::*;
use resources::DebugUIUpdateTimer;

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Built-in
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            // Resources
            .init_resource::<DebugUIUpdateTimer>()
            // OnEnter
            .add_system(spawn_debug_ui.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    update_fps_text,
                    update_entity_count_text,
                    update_debug_ui_redraw_timer
                )
                .in_set(OnUpdate(AppState::Game))
            )
            // OnExit
            .add_system(despawn_debug_ui.in_schedule(OnExit(AppState::Game)));
    }
}
