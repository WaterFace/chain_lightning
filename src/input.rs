use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fix_cursor_unlock_web::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::states::{GameState, PauseState};

#[derive(Debug, Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            // bevy doesn't currently handle cursor unlocking well in wasm
            // this sort of hacks around it
            .add_plugins(FixPointerUnlockPlugin)
            .init_resource::<ActionState<InputAction>>()
            .init_resource::<InputSettings>()
            .init_resource::<InputState>()
            .insert_resource(default_input_map())
            .add_systems(
                Update,
                handle_input_state.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource, Debug)]
pub struct InputSettings {
    pub turn_rate: f32,
    pub mouse_sensitivity: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        InputSettings {
            turn_rate: 0.5,
            mouse_sensitivity: 0.05,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct InputState {
    pub locked_cursor: bool,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InputAction {
    MoveForward,
    MoveBackward,
    StrafeLeft,
    StrafeRight,
    TurnLeft,
    TurnRight,
    #[actionlike(Axis)]
    TurnAxis,
    Fire,
    Focus,
    Pause,
}

fn default_input_map() -> InputMap<InputAction> {
    let mut map = InputMap::default();
    map.insert_multiple([
        (InputAction::MoveForward, KeyCode::KeyW),
        (InputAction::MoveBackward, KeyCode::KeyS),
        (InputAction::StrafeLeft, KeyCode::KeyA),
        (InputAction::StrafeRight, KeyCode::KeyD),
    ]);
    map.insert_multiple([(InputAction::Fire, MouseButton::Left)]);
    map.insert_axis(InputAction::TurnAxis, MouseMoveAxis::X);

    map.insert_multiple([
        (InputAction::MoveForward, KeyCode::ArrowUp),
        (InputAction::MoveBackward, KeyCode::ArrowDown),
        (InputAction::TurnLeft, KeyCode::ArrowLeft),
        (InputAction::TurnRight, KeyCode::ArrowRight),
    ]);
    map.insert_multiple([(InputAction::Fire, KeyCode::Space)]);

    map.insert(InputAction::Pause, KeyCode::Escape);
    map.insert(InputAction::Focus, MouseButton::Left);

    map
}

fn handle_input_state(
    mut window: Single<&mut Window, With<bevy::window::PrimaryWindow>>,
    mut input_state: ResMut<InputState>,
    pause_state: Res<State<PauseState>>,
    input: Res<ActionState<InputAction>>,
) {
    if input.just_pressed(&InputAction::Focus) && matches!(pause_state.get(), PauseState::Unpaused)
    {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    match window.cursor_options.grab_mode {
        CursorGrabMode::Confined | CursorGrabMode::Locked => {
            input_state.locked_cursor = true;
        }
        CursorGrabMode::None => {
            input_state.locked_cursor = false;
            window.cursor_options.visible = true;
        }
    }
}
