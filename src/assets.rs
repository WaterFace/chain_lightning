use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressReturningSystem, ProgressTracker};

use crate::states::AppState;

#[derive(Debug, Default)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ProgressPlugin::<AppState>::new()
                .with_state_transition(AppState::PreLoading, AppState::AssetLoading)
                .with_state_transition(AppState::AssetLoading, AppState::Ready),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_loading_state(LoadingState::new(AppState::PreLoading))
        .add_loading_state(LoadingState::new(AppState::AssetLoading))
        .add_systems(
            Update,
            (
                track_fake_long_task.track_progress::<AppState>(),
                print_progress,
            )
                .chain()
                .run_if(in_state(AppState::AssetLoading))
                .after(LoadingStateSet(AppState::AssetLoading)),
        );
    }
}

fn track_fake_long_task(time: Res<Time>) -> Progress {
    const DURATION_LONG_TASK_IN_SECS: f32 = 4.0;
    if time.elapsed_secs() > DURATION_LONG_TASK_IN_SECS {
        info!("Long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}

fn print_progress(
    progress: Res<ProgressTracker<AppState>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    let progress = progress.get_global_progress();
    if progress.done > *last_done {
        *last_done = progress.done;
        info!(
            "[Frame {}] Changed progress: {:?}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                .unwrap_or(0.),
            progress
        );
    }
}

pub trait AssetLoadingExt {
    fn load_asset_on_startup<T: AssetCollection>(&mut self) -> &mut Self;
}

impl AssetLoadingExt for App {
    fn load_asset_on_startup<T: AssetCollection>(&mut self) -> &mut Self {
        self.configure_loading_state(
            LoadingStateConfig::new(AppState::AssetLoading).load_collection::<T>(),
        )
    }
}
