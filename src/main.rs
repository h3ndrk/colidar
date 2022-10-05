use assets::Textures;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use cursor::{update_cursor, Cursor};
use goal::detect_goal;
use input::detect_key_input;
use iyes_loopless::prelude::*;
use lidar_communication::{handle_lidar_data, setup_lidar_communication};

mod assets;
mod cursor;
mod goal;
mod input;
mod lidar_communication;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

const TABLE_WIDTH: f32 = 1200.0;
const TABLE_LENGTH: f32 = 1920.0;
const GOAL_WIDTH: f32 = 300.0;
const GOAL_POST_DIAMETER: f32 = 40.0;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum AppState {
    LoadingAssets,
    SetupWorld,
    ConnectToLidar,
    Game,
}

pub struct Score {
    pub right: usize,
    pub left: usize,
  }


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_loopless_state(AppState::LoadingAssets)
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::SetupWorld)
                .with_collection::<Textures>(),
        )
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "CoLiDAR".to_string(),
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(Score {
            left: 0,
            right: 0,
        })  
        .add_startup_system(setup_camera)
        .init_resource::<Cursor>()
        .add_enter_system(AppState::SetupWorld, setup_table)
        .add_enter_system(AppState::ConnectToLidar, setup_lidar_communication)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(update_cursor)
                // .with_system(update_stick)
                .with_system(detect_key_input)
                .with_system(detect_goal)
                .with_system(handle_lidar_data)
                .with_system(zoom_camera)
                .into(),
        )
        .run();
}

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Puck;

fn update_stick(cursor: Res<Cursor>, mut sticks: Query<&mut Transform, With<Stick>>) {
    for mut stick in &mut sticks {
        stick.translation = cursor.position.extend(1.0);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.0;
    commands.spawn_bundle(camera).insert(ZoomCamera {
        min_scale: 0.1,
        max_scale: 1.0,
        scroll_speed: 0.01,
    });
}

#[derive(Component)]
pub struct ZoomCamera {
    min_scale: f32,
    max_scale: f32,
    scroll_speed: f32,
}

pub fn zoom_camera(
    mut query: Query<(&ZoomCamera, &mut OrthographicProjection)>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if scroll_events.is_empty() {
        return;
    }

    let pixels_per_line = 10.0;
    let scroll = scroll_events
        .iter()
        .map(|event| match event.unit {
            MouseScrollUnit::Pixel => event.y,
            MouseScrollUnit::Line => event.y * pixels_per_line,
        })
        .sum::<f32>();

    for (zoom_camera, mut projection) in &mut query {
        projection.scale = (projection.scale * (1.0 - scroll * zoom_camera.scroll_speed))
            .clamp(zoom_camera.min_scale, zoom_camera.max_scale);
    }
}

fn setup_table(mut commands: Commands, textures: Res<Textures>) {
    info!("Setting up table...");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TABLE_LENGTH, TABLE_WIDTH)),
                ..default()
            },
            texture: textures.table.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::default());
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                ..default()
            },
            texture: textures.center_circle.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GOAL_POST_DIAMETER, GOAL_POST_DIAMETER)),
                ..default()
            },
            texture: textures.goal_post.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -TABLE_LENGTH / 2.0,
            GOAL_WIDTH / 2.0 + GOAL_POST_DIAMETER / 2.0,
            1.0,
        )));
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GOAL_POST_DIAMETER, GOAL_POST_DIAMETER)),
                ..default()
            },
            texture: textures.goal_post.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -TABLE_LENGTH / 2.0,
            -GOAL_WIDTH / 2.0 - GOAL_POST_DIAMETER / 2.0,
            1.0,
        )));
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GOAL_POST_DIAMETER, GOAL_POST_DIAMETER)),
                ..default()
            },
            texture: textures.goal_post.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            TABLE_LENGTH / 2.0,
            GOAL_WIDTH / 2.0 + GOAL_POST_DIAMETER / 2.0,
            1.0,
        )));
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GOAL_POST_DIAMETER, GOAL_POST_DIAMETER)),
                ..default()
            },
            texture: textures.goal_post.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            TABLE_LENGTH / 2.0,
            -GOAL_WIDTH / 2.0 - GOAL_POST_DIAMETER / 2.0,
            1.0,
        )));

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(
            vec![
                Vec2::new(-TABLE_LENGTH / 2.0, -GOAL_WIDTH / 2.0),
                Vec2::new(-TABLE_LENGTH / 2.0, -TABLE_WIDTH / 2.0),
                Vec2::new(TABLE_LENGTH / 2.0, -TABLE_WIDTH / 2.0),
                Vec2::new(TABLE_LENGTH / 2.0, -GOAL_WIDTH / 2.0),
            ],
            None,
        ))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::default());
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(
            vec![
                Vec2::new(-TABLE_LENGTH / 2.0, GOAL_WIDTH / 2.0),
                Vec2::new(-TABLE_LENGTH / 2.0, TABLE_WIDTH / 2.0),
                Vec2::new(TABLE_LENGTH / 2.0, TABLE_WIDTH / 2.0),
                Vec2::new(TABLE_LENGTH / 2.0, GOAL_WIDTH / 2.0),
            ],
            None,
        ))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::default());

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Damping {
            linear_damping: 0.1,
            angular_damping: 0.0,
        })
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(0.999))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            texture: textures.puck.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            100.0, 100.0, 2.0,
        )))
        .insert(Puck);

    commands
        .spawn()
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(50.0))
        .insert(Stick)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: textures.stick.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 2.0)));
    commands.insert_resource(NextState(AppState::ConnectToLidar));
}
