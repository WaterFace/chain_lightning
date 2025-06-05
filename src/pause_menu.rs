use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    input::InputAction,
    states::{GameState, PauseState},
};

#[derive(Debug, Default)]
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(
            EguiContextPass,
            pause_menu.run_if(in_state(PauseState::Paused)),
        )
        .add_systems(Update, pause_unpause.run_if(in_state(GameState::InGame)));
    }
}

fn pause_unpause(
    current_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    input: Res<ActionState<InputAction>>,
) {
    if !input.just_pressed(&InputAction::Pause) {
        return;
    }

    match *current_state.get() {
        PauseState::Paused => next_state.set(PauseState::Unpaused),
        PauseState::Unpaused => next_state.set(PauseState::Paused),
    }
}

fn pause_menu(mut contexts: EguiContexts) {
    egui::Window::new("Paused").show(contexts.ctx_mut(), |_ui| {});
}
