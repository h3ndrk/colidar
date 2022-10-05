use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use iyes_loopless::state::NextState;
use scrapinator::Lidar;
use std::thread::spawn;

use crate::{
    app_state::{AppState, GameState},
    stick::{LeftStick, RightStick},
    TABLE_LENGTH, TABLE_WIDTH,
};

#[derive(Component)]
pub struct LidarSettings {
    pub pixels_per_meter: f32,
}

pub struct LidarChannel {
    pub receiver: flume::Receiver<Vec<usize>>,
}

pub fn setup_lidar_communication(mut commands: Commands) {
    let (sender, receiver) = flume::unbounded();
    spawn(move || {
        info!("Starting communication thread...");
        let mut lidar = Lidar::connect("192.168.0.1:2112");
        loop {
            let data = lidar.poll_data();
            sender.send(data).unwrap();
        }
    });
    commands.insert_resource(LidarChannel { receiver });
    commands.insert_resource(LidarSettings {
        pixels_per_meter: 1300.0,
    });
}

pub fn wait_for_lidar_messages(mut commands: Commands, lidar_channel: Res<LidarChannel>) {
    if lidar_channel
        .receiver
        .try_iter()
        .any(|message| !message.is_empty())
    {
        commands.insert_resource(NextState(AppState::Calibration))
    }
}

pub fn lidar_calibration(
    mut commands: Commands,
    lidar_channel: Res<LidarChannel>,
    mut lidar_settings: ResMut<LidarSettings>,
) {
    if let Ok(rays) = lidar_channel.receiver.try_recv() {
        info!("Calibrating...");
        let points = process_lidar_message(rays, 1.0);
        let origin = Vec2::new(0.0, -TABLE_WIDTH / 2.0);
        let closest = points
            .iter()
            .min_by(|&left, &right| {
                let left_distance = (*left - origin).length();
                let right_distance = (*right - origin).length();
                left_distance.total_cmp(&right_distance)
            })
            .unwrap();
        info!("Closest: {}", closest);
        let distance = origin - *closest;
        dbg!(distance);
        lidar_settings.pixels_per_meter = (TABLE_WIDTH / 2.0) / distance.length();
        dbg!(lidar_settings.pixels_per_meter);
        commands.insert_resource(NextState(AppState::Game(GameState::Running)));
        //commands.insert_resource(NextState(AppState::Tracker));
    }
}

pub fn process_lidar_message(rays: Vec<usize>, pixels_per_meter: f32) -> Vec<Vec2> {
    //let layer_index = 0;
    rays.into_iter()
        .filter(|&distance| distance != 0)
        .enumerate()
        .map(|(i, distance)| {
            let distance = distance as f32;
            let theta = -138.0 + 0.75 * i as f32;
            let x = distance * theta.to_radians().cos();
            let y = distance * theta.to_radians().sin();
            Vec2::new(
                -y * pixels_per_meter,
                x * pixels_per_meter - TABLE_WIDTH / 2.0,
            )
        })
        .filter(|point| {
            let table_x = -TABLE_LENGTH / 2.0..TABLE_LENGTH / 2.0;
            let table_y = -TABLE_WIDTH / 2.0..TABLE_WIDTH / 2.0;
            table_x.contains(&point.x) && table_y.contains(&point.y)
        })
        .collect()
}

pub fn scale_lidar(mut lidar_settings: ResMut<LidarSettings>, keyboard_input: Res<Input<KeyCode>>) {
    let scale = if keyboard_input.pressed(KeyCode::J) {
        -1.0
    } else if keyboard_input.pressed(KeyCode::K) {
        1.0
    } else {
        0.0
    };
    lidar_settings.pixels_per_meter += 2.0 * scale;
}

pub struct Cluster {
    pub center: Vec2,
    pub points: Vec<Vec2>,
}

impl Cluster {
    pub fn new(center: Vec2) -> Self {
        Self {
            center,
            points: vec![center],
        }
    }

    pub fn add(&mut self, point: Vec2) {
        self.points.push(point);
        let length = self.points.len();
        self.center = self.points.iter().sum::<Vec2>() / length as f32;
    }
}

pub fn handle_lidar_data(
    lidar_settings: Res<LidarSettings>,
    lidar_channel: Res<LidarChannel>,
    mut lines: ResMut<DebugLines>,
    mut left_stick: Query<&mut Transform, (With<LeftStick>, Without<RightStick>)>,
    mut right_stick: Query<&mut Transform, (Without<LeftStick>, With<RightStick>)>,
) {
    if let Some(rays) = lidar_channel.receiver.try_iter().last() {
        let mut clusters = Vec::<Cluster>::new();
        if rays.is_empty() {
            return;
        }
        let mut points: Vec<_> = process_lidar_message(rays, lidar_settings.pixels_per_meter);
        // for points in points.windows(2) {
        //     let left = points[0];
        //     let right = points[1];
        //     lines.line_colored(
        //         Vec3::new(left.x, left.y, 1.0),
        //         Vec3::new(right.x, right.y, 1.0),
        //         0.0,
        //         Color::BLUE,
        //     );
        // }
        points.sort_unstable_by(|left, right| {
            let origin = Vec2::new(0.0, -TABLE_WIDTH / 2.0);
            let left_distance = (*left - origin).length();
            let right_distance = (*right - origin).length();
            left_distance.total_cmp(&right_distance)
        });
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
        let low_pass = 0.95;
        let left = clusters
            .iter()
            .find(|position| position.center.x.is_sign_negative());
        let mut left_stick = left_stick.single_mut();
        if let Some(cluster) = left {
            left_stick.translation.x =
                (1.0 - low_pass) * left_stick.translation.x + low_pass * cluster.center.x;
            left_stick.translation.y =
                (1.0 - low_pass) * left_stick.translation.y + low_pass * cluster.center.y;
        }

        let right = clusters
            .iter()
            .find(|position| position.center.x.is_sign_positive());
        let mut right_stick = right_stick.single_mut();
        if let Some(cluster) = right {
            right_stick.translation.x =
                (1.0 - low_pass) * right_stick.translation.x + low_pass * cluster.center.x;
            right_stick.translation.y =
                (1.0 - low_pass) * right_stick.translation.y + low_pass * cluster.center.y;
        }
    } else {
        warn!("No messages in the channel");
    }
}
