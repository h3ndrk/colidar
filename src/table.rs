use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::Textures, GOAL_POST_DIAMETER, GOAL_WIDTH, TABLE_LENGTH, TABLE_WIDTH};

pub fn setup_table(mut commands: Commands, textures: Res<Textures>) {
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
}
