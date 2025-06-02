use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use character_controller::PlayerPlugin;
use input::InputPlugin;
use physics::PhysicsPlugin;

mod character_controller;
mod input;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((PlayerPlugin, InputPlugin, PhysicsPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 50.0,
            },
            ..OrthographicProjection::default_3d()
        }),
    ));
}
