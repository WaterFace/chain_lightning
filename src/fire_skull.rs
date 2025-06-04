use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_sprite3d::prelude::*;

use crate::{
    character_controller::{CharacterController, CharacterControllerState},
    sprite::{AnimatedSprite3d, FaceCamera},
    states::{AssetLoadingExt, GameState},
};

#[derive(Debug, Default, Component)]
#[require(
    Visibility,
    CharacterController = CharacterController { max_speed: 5.0, acceleration: 10.0 },
)]
pub struct FireSkull {}

#[derive(Debug, Default, Component)]
#[require(Transform, Visibility)]
struct FireSkullVisualRoot;

#[derive(Debug, Default, Component)]
struct SkullVisual;

fn spawn_fire_skull_visuals(
    mut commands: Commands,
    visuals: Res<FireSkullVisuals>,
    query: Query<Entity, Added<FireSkull>>,
    mut sprite3d_params: Sprite3dParams,
) {
    for entity in query.iter() {
        let visual_root = commands
            .spawn((FireSkullVisualRoot, FaceCamera::default()))
            .with_children(|s| {
                let atlas = TextureAtlas {
                    layout: visuals.skull_atlas_layout.clone(),
                    index: 0,
                };
                let animation = AnimatedSprite3d {
                    current: 0,
                    frames: vec![0, 1],
                    timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                };
                let skull = Sprite3dBuilder {
                    image: visuals.skull_atlas_texture.clone(),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    pixels_per_metre: 128.0,
                    ..Default::default()
                }
                .bundle_with_atlas(&mut sprite3d_params, atlas);
                s.spawn((skull, animation));

                let atlas = TextureAtlas {
                    layout: visuals.fire_atlas_layout.clone(),
                    index: 0,
                };
                let animation = AnimatedSprite3d {
                    current: 0,
                    frames: vec![13, 14, 15, 16, 17, 18, 19, 20, 21],
                    timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                };
                let fire = Sprite3dBuilder {
                    image: visuals.fire_atlas_texture.clone(),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    pixels_per_metre: 128.0,
                    ..Default::default()
                }
                .bundle_with_atlas(&mut sprite3d_params, atlas);
                s.spawn((
                    fire,
                    animation,
                    Transform::from_xyz(0.0, 0.4, 0.05).with_scale(Vec3::splat(2.0)),
                ));
            })
            .id();

        commands.entity(entity).add_child(visual_root);
    }
}

#[derive(Resource, AssetCollection)]
struct FireSkullVisuals {
    #[asset(path = "skull_atlas.png")]
    skull_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 1, rows = 2))]
    skull_atlas_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "flame_fire.png")]
    fire_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 5))]
    fire_atlas_layout: Handle<TextureAtlasLayout>,
}

fn move_skulls(
    player_transform: Single<&GlobalTransform, With<crate::player::Player>>,
    mut skull_query: Query<
        (
            &GlobalTransform,
            &CharacterController,
            &mut CharacterControllerState,
        ),
        (With<FireSkull>, Without<crate::player::Player>),
    >,
) {
    let player_pos = player_transform.translation();
    for (skull_transform, controller, mut state) in skull_query.iter_mut() {
        let dir = (player_pos - skull_transform.translation()).normalize_or_zero();

        state.desired_velocity = dir * controller.max_speed;
    }
}

#[derive(Debug, Default)]
pub struct FireSkullPlugin;

impl Plugin for FireSkullPlugin {
    fn build(&self, app: &mut App) {
        // TODO: do this in a proper loading step
        app.load_asset_on_startup::<FireSkullVisuals>().add_systems(
            Update,
            (
                spawn_fire_skull_visuals,
                move_skulls.in_set(bevy_rapier3d::plugin::PhysicsSet::SyncBackend),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
