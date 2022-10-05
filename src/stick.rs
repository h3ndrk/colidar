use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::assets::Textures;

#[derive(Component)]
pub struct Stick;

pub fn setup_stick(mut commands: Commands, textures: Res<Textures>) {
    commands
        .spawn()
        .insert(Stick)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(60.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: textures.stick.clone(),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1000.0, 2.0)));
}
