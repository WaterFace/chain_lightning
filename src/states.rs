use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    AssetLoading,
    Ready,
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Startup,
    InGame,
}

#[derive(Debug, Default)]
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameState>()
            .add_systems(
                OnTransition {
                    exited: AppState::AssetLoading,
                    entered: AppState::Ready,
                },
                |mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::InGame);
                },
            );
    }
}

pub trait AssetLoadingExt {
    fn load_asset_on_startup<T: AssetCollection>(&mut self) -> &mut Self;
}

impl AssetLoadingExt for App {
    fn load_asset_on_startup<T: AssetCollection>(&mut self) -> &mut Self {
        self.add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Ready)
                .load_collection::<T>(),
        )
    }
}
