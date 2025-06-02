use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input::{InputSettings, PlayerAction};
use leafwing_input_manager::prelude::ActionState;

#[derive(Debug, Component)]
#[require(
    CharacterControllerState,
    RigidBody,
    TransformInterpolation,
    Velocity::zero(),
    Collider::ball(1.0)
)]
pub struct CharacterController {
    acceleration: f32,
    max_speed: f32,
}

impl Default for CharacterController {
    fn default() -> Self {
        CharacterController {
            max_speed: 35.0,
            acceleration: 10.0,
        }
    }
}

#[derive(Component, Debug, Default)]
#[require(Transform)]
struct CharacterControllerState {
    // acceleration: f32,
    // max_speed: f32,
    heading: f32,

    desired_turn: f32,
    desired_velocity: Vec2,
}

impl CharacterControllerState {
    fn heading_vec2(&self) -> Vec2 {
        Vec2::from_angle(self.heading)
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(CharacterController::default());
}

fn debug_draw_player(
    query: Query<(
        &CharacterController,
        &CharacterControllerState,
        &GlobalTransform,
    )>,
    mut gizmos: Gizmos,
) {
    for (_player, physics_state, transform) in query.iter() {
        gizmos.arrow(
            transform.translation(),
            transform.translation() + physics_state.heading_vec2().extend(0.0) * 2.0,
            bevy::color::palettes::basic::GREEN,
        );
    }
}

#[derive(Debug, Default, Resource)]
struct AccumulatedInput {
    // movement in player's frame of reference
    movement: Vec2,
    turn: f32,
}

impl AccumulatedInput {
    fn clear(&mut self) {
        self.movement = Vec2::ZERO;
        self.turn = 0.0;
    }
}

fn handle_input(
    mut accumulated: ResMut<AccumulatedInput>,
    input: Res<ActionState<PlayerAction>>,
    input_settings: Res<InputSettings>,
    mut query: Query<(&CharacterController, &mut CharacterControllerState)>,
) {
    // movement is oriented as if the player is facing right
    if input.pressed(&PlayerAction::MoveForward) {
        accumulated.movement += Vec2::X;
    }
    if input.pressed(&PlayerAction::MoveBackward) {
        accumulated.movement += Vec2::NEG_X;
    }

    if input.pressed(&PlayerAction::StrafeLeft) {
        accumulated.movement += Vec2::Y;
    }
    if input.pressed(&PlayerAction::StrafeRight) {
        accumulated.movement += Vec2::NEG_Y;
    }

    if input.pressed(&PlayerAction::TurnLeft) {
        accumulated.turn += 1.0;
    }
    if input.pressed(&PlayerAction::TurnRight) {
        accumulated.turn -= 1.0;
    }

    accumulated.movement = accumulated.movement.normalize_or_zero();

    for (player, mut physics_state) in query.iter_mut() {
        // Allow less-than-full-speed movement, but still normalize if necessary so things don't move
        // faster diagonally
        let desired_movement = if accumulated.movement.length_squared() > 1.0 {
            accumulated.movement.normalize()
        } else {
            accumulated.movement
        };

        physics_state.desired_velocity =
            (desired_movement * player.max_speed).rotate(physics_state.heading_vec2());

        physics_state.desired_turn = accumulated.turn * input_settings.turn_rate;
    }
}

fn advance_physics(
    time: Res<Time<Fixed>>,
    mut query: Query<(
        &CharacterController,
        &mut CharacterControllerState,
        &mut Velocity,
    )>,
    mut accumulated: ResMut<AccumulatedInput>,
) {
    let dt = time.delta_secs();

    for (controller, mut physics_state, mut velocity) in query.iter_mut() {
        use std::f32::consts::PI;
        let diff = physics_state.desired_velocity - velocity.linvel;
        velocity.linvel += diff * controller.acceleration * dt;
        info!("{}", velocity.linvel);

        physics_state.heading += physics_state.desired_turn * 2.0 * PI * dt;

        accumulated.clear();
    }
}

#[derive(Debug, Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(PostUpdate, debug_draw_player)
            .add_systems(
                Update,
                (handle_input, advance_physics)
                    .chain()
                    .in_set(PhysicsSet::SyncBackend),
            )
            .insert_resource(AccumulatedInput::default());
    }
}
