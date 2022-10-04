use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use cursor::{update_cursor, Cursor};

mod cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "CoLiDAR".to_string(),
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .init_resource::<Cursor>()
        .add_system(update_cursor)
        .add_system(update_stick)
        .run();
}

#[derive(Component)]
struct Stick;

fn update_stick(cursor: Res<Cursor>, mut sticks: Query<&mut Transform, With<Stick>>) {
    for mut stick in &mut sticks {
        stick.translation = cursor.position.extend(0.0);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 200.0))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(350.0, 0.0, 0.0)));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 200.0))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-350.0, 0.0, 0.0)));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(400.0, 10.0))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(400.0, 10.0))
        .insert(Restitution::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));

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
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 0.0)));

    commands
        .spawn()
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(50.0))
        .insert(Stick)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
