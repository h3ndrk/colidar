use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::assets::Textures;

#[derive(Component)]
pub struct Puck;

pub fn setup_puck(mut commands: Commands, textures: Res<Textures>) {
    commands
        .spawn()
        .insert(Puck)
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
        .insert(Ccd::enabled());
}
