use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::Textures, STICK_DIAMETER};

#[derive(Component)]
pub struct LeftStick;

#[derive(Component)]
pub struct RightStick;

pub fn setup_stick(mut commands: Commands, textures: Res<Textures>) {
    commands
        .spawn()
        .insert(LeftStick)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(STICK_DIAMETER / 2.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(STICK_DIAMETER, STICK_DIAMETER)),
                ..default()
            },
            texture: textures.stick.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -1000.0, 0.0, 2.0,
        )));

    commands
        .spawn()
        .insert(RightStick)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(STICK_DIAMETER / 2.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(STICK_DIAMETER, STICK_DIAMETER)),
                ..default()
            },
            texture: textures.stick.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(1000.0, 0.0, 2.0)));
}
