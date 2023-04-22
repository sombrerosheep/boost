use super::components::*;
use bevy::prelude::*;

pub fn spawn_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    print!("spawning debug ui");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            DebugUI {},
        ))
        .with_children(|parent| {
            parent.spawn((
                // Entity Count
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Regular.otf"),
                                font_size: 16.0,
                                color: Color::rgb(0.1, 0.6, 0.15),
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..default()
                    },
                    ..default()
                },
                EntityCountText {},
            ));

            // Frame Rate
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Regular.otf"),
                                font_size: 16.0,
                                color: Color::rgb(0.1, 0.6, 0.15),
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..default()
                    },
                    ..default()
                },
                FrameRateText {},
            ));
        });
}

pub fn despawn_debug_ui(mut commands: Commands, debug_query: Query<Entity, With<DebugUI>>) {
    for entity in debug_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
