use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::lidar_communication::{process_lidar_message, Cluster, LidarChannel, LidarSettings};

#[derive(Default)]
pub struct Buffer {
    data: VecDeque<Vec2>,
}

pub fn track_object(
    lidar_settings: Res<LidarSettings>,
    lidar_channel: Res<LidarChannel>,
    mut buffer: Local<Buffer>,
    mut lines: ResMut<DebugLines>,
) {
    for positions in lidar_channel.receiver.try_iter() {
        if buffer.data.len() > 100 {
            buffer.data.pop_front();
        }
        if positions.is_empty() {
            continue;
        }
        let mut points: Vec<_> = process_lidar_message(positions, lidar_settings.pixels_per_meter);
        let mut clusters = Vec::<Cluster>::new();
        for &point in &points {
            let is_close_to_cluster = clusters
                .iter_mut()
                .find(|cluster| (cluster.center - point).length() < 100.0);
            if let Some(cluster) = is_close_to_cluster {
                cluster.add(point);
                continue;
            }
            clusters.push(Cluster::new(point));
        }
        if let Some(cluster) = clusters.first() {
            buffer.data.push_back(cluster.center)
        }
    }
    for points in buffer.data.make_contiguous().windows(2) {
        let left = points[0];
        let right = points[1];
        lines.line_colored(left.extend(1.0), right.extend(1.0), 10.0, Color::RED);
    }
}
