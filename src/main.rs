use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod audio;
mod camera;
mod character_controller;
mod explosion;
mod fire_skull;
mod health;
mod hud;
mod input;
mod level;
mod pause_menu;
mod physics;
mod player;
mod rand;
mod score;
mod shotgun;
mod spawner;
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
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Exploding Skulls".into(),
                        name: Some("bevy.app".into()),
                        resolution: (1280., 720.).into(),
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        ..default()
                    }),
                    ..default()
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
            spawner::SpawnerPlugin,
            rand::RandPlugin,
            score::ScorePlugin,
        ))
        // too many plugins, starting another tuple
        .add_plugins((hud::HudPlugin, pause_menu::PauseMenuPlugin))
        .add_systems(OnEnter(states::GameState::InGame), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(player::Player::default());
}
