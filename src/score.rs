use bevy::prelude::*;

use crate::states::GameState;

#[derive(Debug, Default)]
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::InGame), |mut commands: Commands| {
                commands.insert_resource(Score::default());
            })
            .add_state_scoped_event::<ScoreEvent>(GameState::InGame)
            .add_systems(Update, handle_score_event);
    }
}

#[derive(Debug, Default, Resource)]
pub struct Score {
    pub score: u64,
}

#[derive(Debug, Event)]
pub struct ScoreEvent {
    pub chain: u64,
}

pub const SCORE_PER_SKULL: u64 = 150;
pub const SCORE_PER_CHAIN: u64 = 60;

fn handle_score_event(mut score: ResMut<Score>, mut reader: EventReader<ScoreEvent>) {
    let mut total = 0;
    for ScoreEvent { chain } in reader.read() {
        total += SCORE_PER_SKULL + SCORE_PER_CHAIN * chain
    }
    score.score += total;
}
