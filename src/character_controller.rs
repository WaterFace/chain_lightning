use bevy::prelude::*;

use crate::input::{InputSettings, PlayerAction};
use leafwing_input_manager::prelude::ActionState;

#[derive(Debug, Component)]
#[require(CharacterControllerState)]
pub struct CharacterController {
    acceleration: f32,
    max_speed: f32,
}

impl Default for CharacterController {
    fn default() -> Self {
        CharacterController {
            max_speed: 250.0,
            acceleration: 10.0,
        }
    }
}

#[derive(Component, Debug, Default)]
#[require(Transform)]
struct CharacterControllerState {
    acceleration: f32,
    max_speed: f32,
    velocity: Vec2,
    heading: f32,

    desired_turn: f32,
    desired_velocity: Vec2,

    translation: Vec3,
    previous_translation: Vec3,
}

impl CharacterControllerState {
    fn on_add(
        trigger: Trigger<OnAdd, CharacterControllerState>,
        mut query: Query<(
            &mut CharacterControllerState,
            &Transform,
            &CharacterController,
        )>,
    ) {
        let (mut physics_state, transform, player) =
            query.get_mut(trigger.target()).unwrap_or_else(|e| {
                panic!("failed to query for newly added player physics state: {e}");
            });

        info!(
            "added PlayerPhysicsState with translation: {}",
            transform.translation
        );
        physics_state.translation = transform.translation;
        physics_state.previous_translation = transform.translation;
        physics_state.acceleration = player.acceleration;
        physics_state.max_speed = player.max_speed;
    }

    fn update(&mut self, dt: f32) {
        use std::f32::consts::PI;
        self.previous_translation = self.translation;
        self.translation += self.velocity.extend(0.0) * dt;

        let diff = self.desired_velocity - self.velocity;
        self.velocity += diff * self.acceleration * dt;

        self.heading += self.desired_turn * 2.0 * PI * dt;
    }

    fn set_from_input(&mut self, movement: Vec2, turn: f32) {
        // Allow less-than-full-speed movement, but still normalize if necessary so things don't move
        // faster diagonally
        let desired_movement = if movement.length_squared() > 1.0 {
            movement.normalize()
        } else {
            movement
        };

        self.desired_velocity = (desired_movement * self.max_speed).rotate(self.heading_vec2());

        self.desired_turn = turn;
    }

    fn heading_vec2(&self) -> Vec2 {
        Vec2::from_angle(self.heading)
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        CharacterController::default(),
        Transform::from_xyz(-200.0, 150.0, 0.0),
    ));
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
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation().truncate()),
            5.0,
            bevy::color::palettes::basic::GREEN,
        );
        gizmos.arrow_2d(
            transform.translation().truncate(),
            transform.translation().truncate() + physics_state.heading_vec2() * 15.0,
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

    for (_player, mut physics_state) in query.iter_mut() {
        physics_state.set_from_input(
            accumulated.movement,
            accumulated.turn * input_settings.turn_rate,
        );
    }
}

fn advance_physics(
    time: Res<Time<Fixed>>,
    mut query: Query<&mut CharacterControllerState>,
    mut accumulated: ResMut<AccumulatedInput>,
) {
    for mut physics_state in query.iter_mut() {
        physics_state.update(time.delta_secs());

        accumulated.clear();
    }
}

fn interpolate_rendered_transform(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &CharacterControllerState)>,
) {
    for (mut transform, physics_state) in query.iter_mut() {
        let curr = physics_state.translation;
        let prev = physics_state.previous_translation;

        let alpha = time.overstep_fraction();

        transform.translation = prev.lerp(curr, alpha);
    }
}

#[derive(Debug, Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, debug_draw_player)
            .add_systems(FixedUpdate, advance_physics)
            .add_systems(
                RunFixedMainLoop,
                (
                    handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                    interpolate_rendered_transform
                        .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                ),
            )
            .add_observer(CharacterControllerState::on_add)
            .insert_resource(AccumulatedInput::default());
    }
}
