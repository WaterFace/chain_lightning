use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod audio;
mod camera;
mod character_controller;
mod explosion;
mod fire_skull;
mod health;
mod input;
mod level;
mod physics;
mod player;
mod shotgun;
mod sprite;
mod states;

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
        // Add the states plugin first so asset loading is ready for any other plugin
        .add_plugins(states::StatesPlugin)
        .add_plugins((
            audio::AudioPlugin,
            player::PlayerPlugin,
            character_controller::CharacterControllerPlugin,
            input::InputPlugin,
            physics::PhysicsPlugin,
            camera::CameraPlugin,
            fire_skull::FireSkullPlugin,
            sprite::SpritePlugin,
            shotgun::ShotgunPlugin,
            health::HealthPlugin,
            explosion::ExplosionPlugin,
            level::LevelPlugin,
        ))
        .add_systems(OnEnter(states::GameState::InGame), setup)
        .run();
}

fn setup(mut commands: Commands) {
    // const N: usize = 10;
    // for i in 0..N {
    //     let t = (i + 1) as f32 / (N + 1) as f32;
    //     let p = Quat::from_axis_angle(Vec3::Y, t * 2.0 * std::f32::consts::PI)
    //         * Vec3::new(0.0, 0.0, -3.0);
    //     commands.spawn((
    //         fire_skull::FireSkull::default(),
    //         Transform::from_translation(p + Vec3::new(0.0, 0.0, -13.0)),
    //     ));
    // }
    commands.spawn(player::Player::default());
}
