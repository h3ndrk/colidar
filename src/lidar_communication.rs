use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use iyes_loopless::state::NextState;
use rand::{thread_rng, Rng};
use std::{
    iter::repeat_with,
    thread::{sleep, spawn},
    time::Duration,
};

use crate::{AppState, Stick};

pub struct LidarChannel {
    receiver: flume::Receiver<Vec<Vec2>>,
}

pub fn setup_lidar_communication(mut commands: Commands) {
    let (sender, receiver) = flume::unbounded();
    spawn(move || {
        let mut rng = thread_rng();
        loop {
            sleep(Duration::from_millis(1000));
            let rays = repeat_with(|| {
                let x = rng.gen_range(0.0..400.0);
                let y = rng.gen_range(-200.0..200.0);
                Vec2::new(x, y)
            })
            .take(100)
            .collect();
            sender.send(rays).unwrap();
        }
    });
    commands.insert_resource(LidarChannel { receiver });
    commands.insert_resource(NextState(AppState::Pause));
}

pub fn handle_lidar_data(
    lidar_channel: Res<LidarChannel>,
    mut lines: ResMut<DebugLines>,
    //mut sticks: Query<&mut Transform, With<Stick>>,
) {
    for positions in lidar_channel.receiver.try_iter() {
        for points in positions.windows(2) {
            let left = points[0];
            let right = points[1];
            lines.line_colored(left.extend(0.0), right.extend(0.0), 1.0, Color::RED);
        }
    }
}
