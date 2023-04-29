use bevy::prelude::*;

#[derive(Component)]
pub struct Drifter {
    pub velocity: Vec2,
    pub speed: f32,
}
