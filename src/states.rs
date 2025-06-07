use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameState>()
            .init_state::<PauseState>()
            .add_systems(
                OnTransition {
                    exited: AppState::AssetLoading,
                    entered: AppState::Ready,
                },
                |mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::MainMenu);
                },
            );
    }
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    PreLoading,
    AssetLoading,
    Ready,
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Startup,
    MainMenu,
    InGame,
    End,
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
#[states(scoped_entities)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
