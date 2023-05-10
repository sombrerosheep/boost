use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const FUEL_GAUGE_MARGIN: f32 = 5.0;
const FUEL_GAUGE_MARGIN_X2: f32 = FUEL_GAUGE_MARGIN * 2.0;
const FUEL_GAUGE_WIDTH: f32 = 20.0;
pub const FUEL_GAUGE_HEIGHT: f32 = 100.0;

pub fn spawn_gamehud_ui(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x = window.width() - FUEL_GAUGE_WIDTH - FUEL_GAUGE_MARGIN * 3.0;
    let y = FUEL_GAUGE_MARGIN;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(x),
                        right: Val::Px(x + FUEL_GAUGE_WIDTH + FUEL_GAUGE_MARGIN_X2),
                        top: Val::Px(y),
                        bottom: Val::Px(y + FUEL_GAUGE_HEIGHT + FUEL_GAUGE_MARGIN_X2),
                    },
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::FlexEnd,
                    size: Size::new(
                        Val::Px(FUEL_GAUGE_WIDTH + FUEL_GAUGE_MARGIN_X2),
                        Val::Px(FUEL_GAUGE_HEIGHT + FUEL_GAUGE_MARGIN_X2),
                    ),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            GameHud {},
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(FUEL_GAUGE_WIDTH), Val::Px(FUEL_GAUGE_HEIGHT)),
                        margin: UiRect {
                            top: Val::Px(FUEL_GAUGE_MARGIN),
                            bottom: Val::Px(FUEL_GAUGE_MARGIN),
                            left: Val::Px(FUEL_GAUGE_MARGIN),
                            right: Val::Px(FUEL_GAUGE_MARGIN),
                        },
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                },
                FuelLevelGauge {},
            ));
        });
}

pub fn despawn_gamehud_ui(mut commands: Commands, gamehud_query: Query<Entity, With<GameHud>>) {
    for entity in gamehud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
