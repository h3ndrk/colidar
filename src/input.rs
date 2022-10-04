use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::Puck;

pub fn detect_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
    }
}
