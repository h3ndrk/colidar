use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use iyes_loopless::state::{CurrentState, NextState};

use crate::{
    app_state::{AppState, GameState},
    puck::Puck,
    score::Score,
};

pub fn detect_game_key_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<CurrentState<AppState>>,
    mut score: ResMut<Score>,
    mut pucks: Query<(&mut Transform, &mut Velocity), With<Puck>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut velocity) in &mut pucks {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            *velocity = Velocity::zero();
        }
        *score = Score::default();
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        let toggled_state = match app_state.0 {
            AppState::Game(game_state) => match game_state {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
            },
            _ => panic!("Cannot toggle pause state if not in game"),
        };
        commands.insert_resource(NextState(toggled_state));
    }
}
