use bevy::prelude::*;

use super::components::*;

pub fn update_drifter_position(mut drifter_query: Query<(&mut Transform, &Drifter)>) {
    for (mut transform, drifter) in drifter_query.iter_mut() {
        let movement = drifter.velocity * drifter.speed;

        transform.translation += Vec3::new(movement.x, movement.y, 0.0);
    }
}

pub fn update_tumbler_rotation(
    mut tumbler_query: Query<(&mut Transform, &Tumbler)>,
    time: Res<Time>,
) {
    for (mut transform, tumbler) in tumbler_query.iter_mut() {
        transform.rotate_z(tumbler.rotation_speed * time.delta_seconds());
    }
}
