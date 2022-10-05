use bevy::prelude::*;

use crate::{assets::Fonts, score::Score, SCORE_FONT_SIZE, TABLE_WIDTH};

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
struct GameTimeUi;

pub fn setup_ui(mut commands: Commands, fonts: Res<Fonts>) {
    // Setting up the score
    let score_text_style = TextStyle {
        font: fonts.arial.clone(),
        font_size: SCORE_FONT_SIZE,
        color: Color::BLACK,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0:0", score_text_style)
                .with_alignment(TextAlignment::TOP_CENTER),
            transform: Transform::from_xyz(0.0, TABLE_WIDTH / 2.0 - 50.0, 10.0),
            ..default()
        })
        .insert(ScoreUi);
}

pub fn update_score_ui(mut scores: Query<&mut Text, With<ScoreUi>>, score: Res<Score>) {
    let score_info = format!("{}:{}", score.left, score.right);
    let mut text = scores.single_mut();
    text.sections[0].value = score_info;
}
