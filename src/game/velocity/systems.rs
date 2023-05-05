use bevy::prelude::*;

use super::components::*;

pub fn apply_velocity_to_transform(mut velocity_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in velocity_query.iter_mut() {
        let movement = velocity.velocity * velocity.speed;

        transform.translation += Vec3::new(movement.x, movement.y, 0.0);
    }
}
