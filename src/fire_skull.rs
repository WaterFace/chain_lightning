use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::prelude::*;

use crate::{
    character_controller::{CharacterController, CharacterControllerState},
    health::Health,
    physics::{ENEMY_GROUP, EXPLOSION_GROUP, PLAYER_GROUP, SHOTGUN_GROUP},
    sprite::{AnimatedSprite3d, FaceCamera},
    states::{AssetLoadingExt, GameState},
};

#[derive(Debug, Default, Component)]
#[require(
    Visibility,
    Health::new(10.0),
    CharacterController = CharacterController { max_speed: 5.0, acceleration: 10.0 },
    CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP | ENEMY_GROUP | SHOTGUN_GROUP | EXPLOSION_GROUP),
    Collider::capsule_y(0.5, 0.25),
)]
pub struct FireSkull {}

#[derive(Debug, Default, Component)]
#[require(Transform, Visibility)]
struct FireSkullVisualRoot {
    t: f32,
}

#[derive(Debug, Default, Component)]
struct FireSkullSkullVisual;

#[derive(Debug, Component)]
struct BaseEntity(Entity);

fn bobbing_animation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut FireSkullVisualRoot)>,
) {
    const FREQUENCY: f32 = 4.3;
    const AMPLITUDE: f32 = 0.05;
    const PERIOD: f32 = (2.0 * std::f32::consts::PI) / FREQUENCY;
    for (mut transform, mut skull) in query.iter_mut() {
        transform.translation.y = f32::sin(skull.t * FREQUENCY) * AMPLITUDE;

        skull.t += time.delta_secs();
        skull.t %= PERIOD;
    }
}

fn spawn_fire_skull_visuals(
    mut commands: Commands,
    visuals: Res<FireSkullAssets>,
    query: Query<Entity, Added<FireSkull>>,
    mut sprite3d_params: Sprite3dParams,
    mut animation_offset: Local<f32>,
) {
    for entity in query.iter() {
        let visual_root = commands
            .spawn((
                FireSkullVisualRoot {
                    t: *animation_offset,
                },
                FaceCamera::default(),
            ))
            .with_children(|s| {
                let atlas = TextureAtlas {
                    layout: visuals.skull_atlas_layout.clone(),
                    index: 0,
                };
                let mut timer = Timer::from_seconds(0.5, TimerMode::Repeating);
                timer.tick(Duration::from_secs_f32(*animation_offset));
                let animation = AnimatedSprite3d {
                    current: 0,
                    frames: vec![0, 1],
                    timer,
                    destroy_when_finished: false,
                };
                let skull = Sprite3dBuilder {
                    image: visuals.skull_atlas_texture.clone(),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    pixels_per_metre: 128.0,
                    ..Default::default()
                }
                .bundle_with_atlas(&mut sprite3d_params, atlas);
                s.spawn((skull, animation, FireSkullSkullVisual, BaseEntity(entity)));

                let atlas = TextureAtlas {
                    layout: visuals.fire_atlas_layout.clone(),
                    index: 0,
                };
                let mut timer = Timer::from_seconds(0.15, TimerMode::Repeating);
                timer.tick(Duration::from_secs_f32(*animation_offset));
                let animation = AnimatedSprite3d {
                    current: 0,
                    frames: vec![13, 14, 15, 16, 17, 18, 19, 20, 21],
                    timer,
                    destroy_when_finished: false,
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
                    BaseEntity(entity),
                ));

                // whatever
                *animation_offset = (*animation_offset + std::f32::consts::PI) % 1000.0;
            })
            .id();

        if let Ok(mut c) = commands.get_entity(entity) {
            c.add_child(visual_root);
        } else {
            commands.entity(visual_root).despawn();
        }
    }
}

#[derive(Resource, AssetCollection)]
struct FireSkullAssets {
    #[asset(path = "textures/skull_atlas.png")]
    skull_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 1, rows = 2))]
    skull_atlas_layout: Handle<TextureAtlasLayout>,

    // use .dds here instead of .png, because for some reason
    // bevy picks a bad image format which is not supported on web
    #[asset(path = "textures/flame_fire.dds")]
    fire_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 5))]
    fire_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FireSkullAssets {
    // frame indicies
    const CLOSED: usize = 0;
    #[allow(unused)]
    const OPEN: usize = 1;
}

#[derive(Debug, Clone, Event)]
pub enum FireSkullEvent {
    Teeth(Entity),
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

fn send_fire_skull_events(
    mut writer: EventWriter<FireSkullEvent>,
    query: Query<(&BaseEntity, &Sprite3d), (Changed<Sprite3d>, With<FireSkullSkullVisual>)>,
) {
    for (base_entity, sprite) in query.iter() {
        if let Some(ref atlas) = sprite.texture_atlas {
            if atlas.index == FireSkullAssets::CLOSED {
                writer.write(FireSkullEvent::Teeth(base_entity.0));
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct FireSkullPlugin;

impl Plugin for FireSkullPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireSkullEvent>()
            .load_asset_on_startup::<FireSkullAssets>()
            .add_systems(
                Update,
                (
                    spawn_fire_skull_visuals,
                    bobbing_animation,
                    send_fire_skull_events,
                    move_skulls.in_set(bevy_rapier3d::plugin::PhysicsSet::SyncBackend),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
