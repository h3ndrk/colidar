use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{Puck, TABLE_LENGTH};

pub fn detect_goal(mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>) {
    for (mut transform, mut velocity) in &mut pucks {
        if transform.translation.x.abs() > TABLE_LENGTH / 2.0 {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
    }
}
