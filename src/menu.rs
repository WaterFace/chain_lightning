use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    assets::AssetLoadingExt,
    input::InputAction,
    states::{AppState, GameState},
};

#[derive(Debug, Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<MenuAssets>()
            // load this first so we can use it on the loading screen:
            .configure_loading_state(
                LoadingStateConfig::new(AppState::PreLoading).load_collection::<LoadingAssets>(),
            )
            .add_systems(OnEnter(AppState::AssetLoading), setup_loading_screen)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, main_menu.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(GameState::End), setup_end_screen)
            .add_systems(Update, end_screen.run_if(in_state(GameState::End)));
    }
}

#[derive(Resource, AssetCollection, Debug)]
struct MenuAssets {
    #[asset(path = "textures/title.png")]
    main_menu: Handle<Image>,
    #[asset(path = "textures/end.png")]
    end: Handle<Image>,
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
    assets: Res<MenuAssets>,
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

fn main_menu(
    mut input: ResMut<ActionState<InputAction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(&InputAction::FireSpace) {
        input.release(&InputAction::FireSpace);
        next_state.set(GameState::InGame);
    }
}

fn setup_end_screen(
    mut commands: Commands,
    assets: Res<MenuAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((Camera2d, StateScoped(GameState::End)));
    commands.spawn((
        Sprite {
            custom_size: Some(window.size()),
            ..Sprite::from_image(assets.end.clone())
        },
        StateScoped(GameState::End),
    ));
}

fn end_screen(
    mut input: ResMut<ActionState<InputAction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(&InputAction::FireSpace) {
        input.release(&InputAction::FireSpace);
        next_state.set(GameState::InGame);
    }
}
