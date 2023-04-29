use bevy::prelude::*;

#[derive(Component)]
pub struct Drifter {
    pub velocity: Vec2,
    pub speed: f32,
}

#[derive(Component)]
pub struct Tumbler {
    pub rotation_speed: f32,
}
