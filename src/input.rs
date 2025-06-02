use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Resource, Debug)]
pub struct InputSettings {
    pub turn_rate: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        InputSettings { turn_rate: 1.0 }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    StrafeLeft,
    StrafeRight,
    TurnLeft,
    TurnRight,
    Fire,
}

fn default_input_map() -> InputMap<PlayerAction> {
    let mut map = InputMap::default();
    map.insert_multiple([
        (PlayerAction::MoveForward, KeyCode::KeyW),
        (PlayerAction::MoveBackward, KeyCode::KeyS),
        (PlayerAction::StrafeLeft, KeyCode::KeyA),
        (PlayerAction::StrafeRight, KeyCode::KeyD),
    ]);
    map.insert_multiple([(PlayerAction::Fire, MouseButton::Left)]);

    map.insert_multiple([
        (PlayerAction::MoveForward, KeyCode::ArrowUp),
        (PlayerAction::MoveBackward, KeyCode::ArrowDown),
        (PlayerAction::TurnLeft, KeyCode::ArrowLeft),
        (PlayerAction::TurnRight, KeyCode::ArrowRight),
    ]);
    map.insert_multiple([(PlayerAction::Fire, KeyCode::Space)]);

    map
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<ActionState<PlayerAction>>()
            .init_resource::<InputSettings>()
            .insert_resource(default_input_map());
    }
}
