use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod camera;
mod character_controller;
mod input;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: bevy::image::ImageSamplerDescriptor::nearest(),
                }),
        )
        .add_plugins((
            player::PlayerPlugin,
            character_controller::CharacterControllerPlugin,
            input::InputPlugin,
            physics::PhysicsPlugin,
            camera::CameraPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(player::Player::default());
    commands.spawn(AmbientLight {
        brightness: 0.5,
        ..Default::default()
    });

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -1.0),
        Mesh3d(meshes.add(Plane3d {
            normal: Dir3::Z,
            half_size: Vec2::ONE * 15.0,
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("checkers.png")),
            ..Default::default()
        })),
    ));
}
