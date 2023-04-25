use super::components::*;
use super::resources::*;
use bevy::diagnostic::*;
use bevy::prelude::*;

// These are held by bevy but don't seem to be exported
// We can loop through diagnostics and find by name
// Seems silly to have the .get method but not make the
// DiagnosticId's accessible?
pub const FPS: DiagnosticId = DiagnosticId::from_u128(288146834822086093791974408528866909483);
pub const ENTITY_COUNT: DiagnosticId =
    DiagnosticId::from_u128(187513512115068938494459732780662867798);

pub fn update_fps_text(
    mut text_query: Query<&mut Text, With<FrameRateText>>,
    ui_redraw_timer: Res<DebugUIUpdateTimer>,
    diagnostics: Res<Diagnostics>,
) {
    if !ui_redraw_timer.timer.finished() {
        return;
    }

    for mut text in text_query.iter_mut() {
        if let Some(diag) = diagnostics.get(FPS) {
            if let Some(fps) = diag.value() {
                text.sections[0].value = format!("FPS: {}", fps);
            }
        }
    }
}

pub fn update_entity_count_text(
    mut text_query: Query<&mut Text, With<EntityCountText>>,
    ui_redraw_timer: Res<DebugUIUpdateTimer>,
    diag: Res<Diagnostics>,
) {
    if !ui_redraw_timer.timer.finished() {
        return;
    }

    for mut text in text_query.iter_mut() {
        if let Some(entity_count) = diag.get(ENTITY_COUNT) {
            if let Some(count) = entity_count.value() {
                text.sections[0].value = format!("Entities: {}", count);
            }
        }
    }
}

pub fn update_debug_ui_redraw_timer(
    mut ui_redraw_timer: ResMut<DebugUIUpdateTimer>,
    time: Res<Time>,
) {
    ui_redraw_timer.timer.tick(time.delta());
}
