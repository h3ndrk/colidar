use assets::Textures;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use cursor::{update_cursor, Cursor};
use iyes_loopless::prelude::*;

mod assets;
mod cursor;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum AppState {
    LoadingAssets,
    SetupWorld,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_loopless_state(AppState::LoadingAssets)
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::SetupWorld)
                .with_collection::<Textures>(),
        )
        //.add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "CoLiDAR".to_string(),
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(setup_camera)
        .init_resource::<Cursor>()
        .add_enter_system(AppState::SetupWorld, setup_table)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(update_cursor)
                .with_system(update_stick)
                .into(),
        )
        .run();
}

#[derive(Component)]
struct Stick;

fn update_stick(cursor: Res<Cursor>, mut sticks: Query<&mut Transform, With<Stick>>) {
    for mut stick in &mut sticks {
        stick.translation = cursor.position.extend(1.0);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_table(mut commands: Commands, textures: Res<Textures>) {
    info!("Setting up table...");
    let width = 400.0;
    let length = 800.0;
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(
            vec![
                Vec2::new(-length / 2.0, -width / 2.0),
                Vec2::new(-length / 2.0, width / 2.0),
                Vec2::new(length / 2.0, width / 2.0),
                Vec2::new(length / 2.0, -width / 2.0),
                Vec2::new(-length / 2.0, -width / 2.0),
            ],
            None,
        ))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(length, width)),
                ..default()
            },
            texture: textures.table.clone(),
            ..default()
        })
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
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            texture: textures.puck.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(100.0, 100.0, 1.0)));

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
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));
    commands.insert_resource(NextState(AppState::Game))
}
