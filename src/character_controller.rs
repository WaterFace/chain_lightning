use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    input::{InputAction, InputSettings, InputState},
    player::Player,
    states::{GameState, PauseState},
};
use leafwing_input_manager::prelude::ActionState;

#[derive(Debug, Default)]
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, set_velocity)
                .chain()
                .in_set(PhysicsSet::SyncBackend)
                .run_if(in_state(GameState::InGame).and(in_state(PauseState::Unpaused))),
        )
        .insert_resource(AccumulatedInput::default());
    }
}

#[derive(Debug, Component)]
#[require(
    Transform,
    CharacterControllerState,
    ReadHeading,
    RigidBody,
    LockedAxes = LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
    TransformInterpolation,
    Velocity::zero(),
    Collider::capsule_y(0.5, 0.5),
)]
pub struct CharacterController {
    pub acceleration: f32,
    pub max_speed: f32,
}

/// Allows reading the current heading of the character controller
///
/// Set by the character controller. Should not be modified directly.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct ReadHeading {
    pub heading: f32,
}

impl ReadHeading {
    pub fn to_vec3(self) -> Vec3 {
        Quat::from_axis_angle(Vec3::Y, self.heading) * (Vec3::NEG_Z)
    }
}

#[derive(Component, Debug, Default)]
pub struct CharacterControllerState {
    pub heading: f32,

    pub desired_turn: f32,
    pub desired_velocity: Vec3,
}

#[derive(Debug, Default, Resource)]
struct AccumulatedInput {
    // movement in player's frame of reference
    movement: Vec3,
    turn: f32,
}

impl AccumulatedInput {
    fn clear(&mut self) {
        self.movement = Vec3::ZERO;
        self.turn = 0.0;
    }
}

fn handle_input(
    mut accumulated: ResMut<AccumulatedInput>,
    input: Res<ActionState<InputAction>>,
    input_settings: Res<InputSettings>,
    input_state: Res<InputState>,
    mut query: Query<(&CharacterController, &mut CharacterControllerState, &Player)>,
) {
    // movement is oriented as if the player is facing in the negative Z direction
    if input.pressed(&InputAction::MoveForward) {
        accumulated.movement += Vec3::NEG_Z;
    }
    if input.pressed(&InputAction::MoveBackward) {
        accumulated.movement += Vec3::Z;
    }

    if input.pressed(&InputAction::StrafeLeft) {
        accumulated.movement += Vec3::NEG_X;
    }
    if input.pressed(&InputAction::StrafeRight) {
        accumulated.movement += Vec3::X;
    }

    if input.pressed(&InputAction::TurnLeft) {
        accumulated.turn += 1.0;
    }
    if input.pressed(&InputAction::TurnRight) {
        accumulated.turn -= 1.0;
    }
    if input_state.locked_cursor {
        if let Some(axis_data) = input.axis_data(&InputAction::TurnAxis) {
            accumulated.turn -= axis_data.value * input_settings.mouse_sensitivity;
        }
    }

    for (controller, mut physics_state, player) in query.iter_mut() {
        if player.dead {
            physics_state.desired_velocity = Vec3::ZERO;
            continue;
        }

        // Allow less-than-full-speed movement, but still normalize if necessary so things don't move
        // faster diagonally
        let desired_movement = if accumulated.movement.length_squared() > 1.0 {
            accumulated.movement.normalize()
        } else {
            accumulated.movement
        };

        physics_state.desired_velocity = Quat::from_axis_angle(Vec3::Y, physics_state.heading)
            * (desired_movement * controller.max_speed);

        physics_state.desired_turn = accumulated.turn * input_settings.turn_rate;
    }
}

fn set_velocity(
    time: Res<Time<Fixed>>,
    mut query: Query<(
        &CharacterController,
        &mut CharacterControllerState,
        &mut ReadHeading,
        &mut Velocity,
    )>,
    mut accumulated: ResMut<AccumulatedInput>,
) {
    let dt = time.delta_secs();

    for (controller, mut physics_state, mut read_heading, mut velocity) in query.iter_mut() {
        use std::f32::consts::PI;
        let diff = physics_state.desired_velocity - velocity.linvel;
        velocity.linvel += diff * controller.acceleration * dt;

        physics_state.heading += physics_state.desired_turn * 2.0 * PI * dt;
        read_heading.heading = physics_state.heading;

        accumulated.clear();
    }
}
