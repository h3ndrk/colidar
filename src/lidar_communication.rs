use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use iyes_loopless::state::NextState;
use sick_scan_xd::{CartesianPoint, SickScanApiHandle};
use std::{thread::spawn, time::Duration};

use crate::{
    app_state::{AppState, GameState},
    stick::Stick,
    TABLE_LENGTH, TABLE_WIDTH,
};

#[derive(Component)]
pub struct LidarSettings {
    pixels_per_meter: f32,
}

pub struct LidarChannel {
    receiver: flume::Receiver<Vec<CartesianPoint>>,
}

pub fn setup_lidar_communication(mut commands: Commands) {
    let (sender, receiver) = flume::unbounded();
    spawn(move || {
        info!("Starting communication thread...");
        SickScanApiHandle::load().unwrap();
        info!("Loaded API Handle");
        let api_handle = SickScanApiHandle::create();
        info!("Created API Handle");
        api_handle.initialize_from_command_line().unwrap();
        info!("Initialized API Handle");
        loop {
            let message = api_handle
                .wait_for_next_cartesian_point_cloud_message(Duration::from_secs(1))
                .unwrap();
            let data = message.get_data();
            //info!("Got Data");
            // for item in data {
            //     println!("data: {:?}", item);
            // }
            // sleep(Duration::from_secs(1));

            sender.send(data).unwrap();
        }
    });
    commands.insert_resource(LidarChannel { receiver });
    commands.insert_resource(LidarSettings {
        pixels_per_meter: 1200.0,
    });
    commands.insert_resource(NextState(AppState::Game(GameState::Running)))
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

pub fn handle_lidar_data(
    lidar_settings: Res<LidarSettings>,
    lidar_channel: Res<LidarChannel>,
    mut lines: ResMut<DebugLines>,
    mut sticks: Query<&mut Transform, With<Stick>>,
) {
    for positions in lidar_channel.receiver.try_iter() {
        // for points in positions.windows(2) {
        //     let left = &points[0];
        //     let right = &points[1];
        //     let left = Vec3::new(-left.y * 1000.0, left.x * 1000.0 - TABLE_WIDTH / 2.0, 1.0);
        //     let right = Vec3::new(-right.y * 1000.0, right.x * 1000.0 - TABLE_WIDTH / 2.0, 1.0);
        //     lines.line_colored(left, right, 0.0, Color::RED);
        // }
        if positions.is_empty() {
            continue;
        }
        let mut points: Vec<_> = positions
            .chunks(4)
            .filter(|points| points[0].x != 0.0 && points[0].y != 0.0)
            .map(|points| {
                let point = &points[0];
                Vec2::new(
                    -point.y * lidar_settings.pixels_per_meter,
                    point.x * lidar_settings.pixels_per_meter - TABLE_WIDTH / 2.0,
                )
            })
            .filter(|point| {
                let table_x = -TABLE_LENGTH / 2.0..TABLE_LENGTH / 2.0;
                let table_y = -TABLE_WIDTH / 2.0..TABLE_WIDTH / 2.0;
                table_x.contains(&point.x) && table_y.contains(&point.y)
            })
            .collect();
        for points in points.windows(2) {
            let left = points[0];
            let right = points[1];
            lines.line_colored(
                Vec3::new(left.x, left.y, 1.0),
                Vec3::new(right.x, right.y, 1.0),
                0.0,
                Color::BLUE,
            );
        }
        points.sort_unstable_by(|left, right| {
            let origin = Vec2::new(0.0, -TABLE_WIDTH / 2.0);
            let left_distance = (*left - origin).length();
            let right_distance = (*right - origin).length();
            left_distance.total_cmp(&right_distance)
        });
        let mut stick = sticks.single_mut();
        //info!("adding: points len: {}", points.)
        let closest = match points.first() {
            Some(value) => value,
            None => continue,
        };
        let low_pass = 0.8;
        stick.translation.x = (1.0 - low_pass) * stick.translation.x + low_pass * closest.x;
        stick.translation.y = (1.0 - low_pass) * stick.translation.y + low_pass * closest.y;
    }
}
