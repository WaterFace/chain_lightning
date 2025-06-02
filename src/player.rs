use bevy::prelude::*;

use crate::input::{InputSettings, PlayerAction};
use leafwing_input_manager::prelude::ActionState;

#[derive(Debug, Default, Component)]
#[require(PlayerPhysicsState)]
pub struct Player {}

#[derive(Component, Debug)]
#[require(Transform)]
struct PlayerPhysicsState {
    velocity: Vec2,
    heading: f32,

    desired_turn: f32,

    translation: Vec3,
    previous_translation: Vec3,
}

impl Default for PlayerPhysicsState {
    fn default() -> Self {
        use std::f32::consts::PI;
        PlayerPhysicsState {
            velocity: Vec2::ZERO,
            heading: PI / 2.0,
            desired_turn: 0.0,
            translation: Vec3::ZERO,
            previous_translation: Vec3::ZERO,
        }
    }
}

impl PlayerPhysicsState {
    fn on_add(
        trigger: Trigger<OnAdd, PlayerPhysicsState>,
        mut query: Query<(&mut PlayerPhysicsState, &Transform)>,
    ) {
        let (mut physics_state, transform) = query.get_mut(trigger.target()).unwrap_or_else(|e| {
            panic!("failed to query for newly added player physics state: {e}");
        });

        info!(
            "added PlayerPhysicsState with translation: {}",
            transform.translation
        );
        physics_state.translation = transform.translation;
        physics_state.previous_translation = transform.translation;
    }

    fn update(&mut self, dt: f32) {
        use std::f32::consts::PI;
        self.previous_translation = self.translation;
        self.translation += self.velocity.extend(0.0) * dt;
        self.heading += self.desired_turn * 2.0 * PI * dt;
    }

    fn set_from_input(&mut self, movement: Vec2, turn: f32) {
        let new_vel = movement.rotate(self.heading_vec2());
        self.velocity = new_vel;
        self.desired_turn = turn;
    }

    fn heading_vec2(&self) -> Vec2 {
        Vec2::from_angle(self.heading)
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((Player::default(), Transform::from_xyz(-200.0, 150.0, 0.0)));
}

fn debug_draw_player(
    query: Query<(&Player, &PlayerPhysicsState, &GlobalTransform)>,
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
    mut query: Query<&mut PlayerPhysicsState>,
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

    for mut physics_state in query.iter_mut() {
        const SPEED: f32 = 150.0;
        physics_state.set_from_input(
            accumulated.movement * SPEED,
            accumulated.turn * input_settings.turn_rate,
        );
    }
}

fn advance_physics(
    time: Res<Time<Fixed>>,
    mut query: Query<&mut PlayerPhysicsState>,
    mut accumulated: ResMut<AccumulatedInput>,
) {
    for mut physics_state in query.iter_mut() {
        physics_state.update(time.delta_secs());

        accumulated.clear();
    }
}

fn interpolate_rendered_transform(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PlayerPhysicsState)>,
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
            .add_observer(PlayerPhysicsState::on_add)
            .insert_resource(AccumulatedInput::default());
    }
}
