use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    input::{InputSettings, PlayerAction},
    states::GameState,
};
use leafwing_input_manager::prelude::ActionState;

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
    input: Res<ActionState<PlayerAction>>,
    input_settings: Res<InputSettings>,
    mut query: Query<
        (&CharacterController, &mut CharacterControllerState),
        With<crate::player::Player>,
    >,
) {
    // movement is oriented as if the player is facing in the negative Z direction
    if input.pressed(&PlayerAction::MoveForward) {
        accumulated.movement += Vec3::NEG_Z;
    }
    if input.pressed(&PlayerAction::MoveBackward) {
        accumulated.movement += Vec3::Z;
    }

    if input.pressed(&PlayerAction::StrafeLeft) {
        accumulated.movement += Vec3::NEG_X;
    }
    if input.pressed(&PlayerAction::StrafeRight) {
        accumulated.movement += Vec3::X;
    }

    if input.pressed(&PlayerAction::TurnLeft) {
        accumulated.turn += 1.0;
    }
    if input.pressed(&PlayerAction::TurnRight) {
        accumulated.turn -= 1.0;
    }

    for (player, mut physics_state) in query.iter_mut() {
        // Allow less-than-full-speed movement, but still normalize if necessary so things don't move
        // faster diagonally
        let desired_movement = if accumulated.movement.length_squared() > 1.0 {
            accumulated.movement.normalize()
        } else {
            accumulated.movement
        };

        physics_state.desired_velocity = Quat::from_axis_angle(Vec3::Y, physics_state.heading)
            * (desired_movement * player.max_speed);

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

#[derive(Debug, Default)]
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, set_velocity)
                .chain()
                .in_set(PhysicsSet::SyncBackend)
                .run_if(in_state(GameState::InGame)),
        )
        .insert_resource(AccumulatedInput::default());
    }
}
