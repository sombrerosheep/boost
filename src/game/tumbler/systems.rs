use bevy::prelude::*;

use super::components::*;

pub fn update_tumbler_rotation(
    mut tumbler_query: Query<(&mut Transform, &Tumbler)>,
    time: Res<Time>,
) {
    for (mut transform, tumbler) in tumbler_query.iter_mut() {
        transform.rotate_z(tumbler.rotation_speed * time.delta_seconds());
    }
}
