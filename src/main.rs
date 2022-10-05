use assets::{Fonts, Textures};
use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use cursor::{update_cursor, Cursor};
use goal::detect_goal;
use input::detect_pause_key_input;
use input::{detect_game_ended_key_input, detect_game_key_input};
use iyes_loopless::prelude::*;
use lidar_communication::{handle_lidar_data, setup_lidar_communication};
use std::fmt;

mod assets;
mod cursor;
mod goal;
mod input;
mod lidar_communication;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

// const TABLE_WIDTH: f32 = 1200.0;
// const TABLE_LENGTH: f32 = 1920.0;
const TABLE_WIDTH: f32 = 600.0;
const TABLE_LENGTH: f32 = 960.0;
const GOAL_WIDTH: f32 = 300.0;
const GOAL_POST_DIAMETER: f32 = 40.0;
const SCORE_FONT_SIZE: f32 = 70.0;
const TIMER_FONT_SIZE: f32 = 55.0;
const GAME_LENGTH: f32 = 10.0;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppState {
    LoadingAssets,
    SetupWorld,
    ConnectToLidar,
    Game,
    Pause,
    Ended,
}

pub struct Score {
    pub right: usize,
    pub left: usize,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let spacing = "       ";
        let mut result = "".to_owned();
        result.push_str(&format!("{}", self.left));
        if self.left < 9 && self.right > 9 {
            result.push_str(" ");
            result.push_str(spacing);
            result.push_str(&format!("{}", self.right))
        } else if self.left > 9 && self.right < 9 {
            result.push_str(spacing);
            result.push_str(" ");
            result.push_str(&format!("{}", self.right))
        } else {
            result.push_str(spacing);
            result.push_str(&format!("{}", self.right))
        }
        write!(f, "{}", result)
    }
}

impl Score {
    fn reset(&mut self) {
        self.left = 0;
        self.right = 0;
    }
}

pub struct RemainingGameTime {
    pub time: f32, // in seconds
}

impl fmt::Display for RemainingGameTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = (self.time / 60.0).trunc();
        let seconds = self.time % 60.0;
        write!(f, "{}:{:.1}", minutes, seconds)
    }
}

impl RemainingGameTime {
    fn reset(&mut self) {
        self.time = GAME_LENGTH;
    }
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
                .with_collection::<Textures>()
                .with_collection::<Fonts>(),
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
        .insert_resource(Score { left: 0, right: 0 })
        .insert_resource(RemainingGameTime { time: GAME_LENGTH })
        .add_startup_system(setup_camera)
        .init_resource::<Cursor>()
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::SetupWorld)
                .with_system(setup_table)
                .with_system(setup_ui)
                .into(),
        )
        .add_enter_system(AppState::ConnectToLidar, setup_lidar_communication)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(update_cursor)
                .with_system(update_stick)
                .with_system(detect_game_key_input)
                .with_system(detect_goal)
                // .with_system(handle_lidar_data)
                .with_system(zoom_camera)
                .with_system(score_update_system)
                .with_system(game_timer_update_system)
                .with_system(advance_timer_system)
                .with_system(detect_game_end)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Pause)
                .with_system(detect_pause_key_input)
                .with_system(score_update_system)
                .with_system(game_timer_update_system)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Ended)
                .with_system(detect_game_ended_key_input)
                .with_system(score_update_system)
                .with_system(game_timer_update_system)
                .into(),
        )
        .run();
}

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Puck;

#[derive(Component)]
struct ScoreUi;

#[derive(Component)]
struct GameTimeUi;

fn score_update_system(mut scores: Query<&mut Text, With<ScoreUi>>, score: Res<Score>) {
    let score_info = score.to_string();
    let mut text = scores.single_mut();
    text.sections[0].value = score_info;
}

fn update_stick(cursor: Res<Cursor>, mut sticks: Query<&mut Transform, With<Stick>>) {
    for mut stick in &mut sticks {
        stick.translation = cursor.position.extend(1.0);
    }
}

fn advance_timer_system(time: Res<Time>, mut game_timer: ResMut<RemainingGameTime>) {
    game_timer.time -= time.delta_seconds();
}

fn game_timer_update_system(
    mut game_timers: Query<&mut Text, With<GameTimeUi>>,
    game_timer: Res<RemainingGameTime>,
) {
    let timer_info = game_timer.to_string();
    let mut text = game_timers.single_mut();
    text.sections[0].value = timer_info;
}

fn detect_game_end(
    mut score: ResMut<Score>,
    mut game_time: ResMut<RemainingGameTime>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
    mut commands: Commands,
) {
    if game_time.time <= 0.0 {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
        score.reset();
        game_time.reset();
        commands.insert_resource(NextState(AppState::Pause))
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

fn setup_ui(
    mut commands: Commands,
    fonts: Res<Fonts>,
    score: Res<Score>,
    game_time: Res<RemainingGameTime>,
) {
    // Setting up the score
    let score_text_style = TextStyle {
        font: fonts.arial.clone(),
        font_size: SCORE_FONT_SIZE,
        color: Color::BLACK,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(score.to_string(), score_text_style.clone())
                .with_alignment(TextAlignment::TOP_CENTER),
            transform: Transform::from_xyz(0.0, TABLE_WIDTH / 2.0 - 50.0, 10.0),
            ..default()
        })
        .insert(ScoreUi);

    // Setting up the game timer
    let game_time_text_style = TextStyle {
        font: fonts.arial.clone(),
        font_size: TIMER_FONT_SIZE,
        color: Color::BLACK,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(game_time.to_string(), game_time_text_style.clone())
                .with_alignment(TextAlignment::TOP_RIGHT),
            transform: Transform::from_xyz(
                TABLE_LENGTH / 2.0 - 15.0,
                TABLE_WIDTH / 2.0 - 15.0,
                10.0,
            ),
            ..default()
        })
        .insert(GameTimeUi);
    commands.insert_resource(NextState(AppState::ConnectToLidar));
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
}
