use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use iyes_loopless::state::NextState;

use crate::{AppState, Puck, RemainingGameTime, Score};

pub fn detect_game_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<Score>,
    mut game_time: ResMut<RemainingGameTime>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
        score.reset();
        game_time.reset();
        commands.insert_resource(NextState(AppState::Pause));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(AppState::Pause));
    }
}

pub fn detect_pause_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<Score>,
    mut game_time: ResMut<RemainingGameTime>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
        score.reset();
        game_time.reset();
        commands.insert_resource(NextState(AppState::Pause));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(AppState::Game));
    }
}

pub fn detect_game_ended_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<Score>,
    mut game_time: ResMut<RemainingGameTime>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
        score.reset();
        game_time.reset();
        commands.insert_resource(NextState(AppState::Pause));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(AppState::Game));
    }
}
