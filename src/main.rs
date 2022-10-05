use app_state::AppState;
use assets::{Fonts, Textures};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use camera::{setup_camera, zoom_camera};
use input::detect_game_key_input;
use iyes_loopless::prelude::*;
use lidar_communication::{handle_lidar_data, scale_lidar, setup_lidar_communication};
use puck::setup_puck;
use score::{detect_goals, Score};
use stick::setup_stick;
use table::setup_table;
use ui::{setup_ui, update_score_ui};

mod app_state;
mod assets;
mod camera;
mod input;
mod lidar_communication;
mod puck;
mod score;
mod stick;
mod table;
mod ui;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

const TABLE_WIDTH: f32 = 1200.0;
const TABLE_LENGTH: f32 = 1920.0;
const GOAL_WIDTH: f32 = 300.0;
const GOAL_POST_DIAMETER: f32 = 40.0;
const SCORE_FONT_SIZE: f32 = 70.0;
//const TIMER_FONT_SIZE: f32 = 55.0;
//const GAME_DURATION: Duration = Duration::from_secs(60);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_loopless_state(AppState::LoadingAssets)
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::Setup)
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
        .add_startup_system(setup_camera)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Setup)
                .with_system(setup_table)
                .with_system(setup_ui)
                .with_system(setup_puck)
                .with_system(setup_stick)
                .with_system(|mut commands: Commands| {
                    commands.insert_resource(NextState(AppState::ConnectToLidar))
                })
                .into(),
        )
        .add_enter_system(AppState::ConnectToLidar, setup_lidar_communication)
        .add_system_set(
            ConditionSet::new()
                .run_if(|app_state: Res<CurrentState<AppState>>| {
                    matches!(app_state.0, AppState::Game(_))
                })
                .with_system(detect_game_key_input)
                .with_system(detect_goals)
                .with_system(handle_lidar_data)
                .with_system(zoom_camera)
                .with_system(scale_lidar)
                .with_system(update_score_ui)
                .into(),
        )
        .run();
}
