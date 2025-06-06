use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    input::InputAction,
    states::{AppState, AssetLoadingExt, GameState},
};

#[derive(Debug, Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<MainMenuAssets>()
            // load this first so we can use it on the loading screen:
            .add_loading_state(
                LoadingState::new(AppState::PreLoading)
                    .continue_to_state(AppState::AssetLoading)
                    .load_collection::<LoadingAssets>(),
            )
            .add_systems(OnEnter(AppState::AssetLoading), setup_loading_screen)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(Resource, AssetCollection, Debug)]
struct MainMenuAssets {
    #[asset(path = "textures/title.png")]
    main_menu: Handle<Image>,
}

#[derive(Resource, AssetCollection, Debug)]
struct LoadingAssets {
    #[asset(path = "textures/loading.png")]
    loading: Handle<Image>,
}

fn setup_loading_screen(
    mut commands: Commands,
    assets: Res<LoadingAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((Camera2d, StateScoped(AppState::AssetLoading)));
    commands.spawn((
        Sprite {
            custom_size: Some(window.size()),
            ..Sprite::from_image(assets.loading.clone())
        },
        StateScoped(AppState::AssetLoading),
    ));
}

fn setup_main_menu(
    mut commands: Commands,
    assets: Res<MainMenuAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((Camera2d, StateScoped(GameState::MainMenu)));
    commands.spawn((
        Sprite {
            custom_size: Some(window.size()),
            ..Sprite::from_image(assets.main_menu.clone())
        },
        StateScoped(GameState::MainMenu),
    ));
}

fn main_menu(input: Res<ActionState<InputAction>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(&InputAction::Fire) {
        next_state.set(GameState::InGame);
    }
}
