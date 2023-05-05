use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
    pub speed: f32,
}
