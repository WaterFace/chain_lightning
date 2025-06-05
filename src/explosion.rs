use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};

use crate::{
    health::{DamageEvent, Health},
    physics::{ENEMY_GROUP, EXPLOSION_GROUP, PLAYER_GROUP},
    sprite::{AnimatedSprite3d, FaceCamera},
    states::{AssetLoadingExt, GameState, PauseState},
};

#[derive(Debug, Default)]
pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEvent>()
            .load_asset_on_startup::<ExplosionAssets>()
            .add_systems(
                Update,
                (spawn_explosion_visual, explosion_collision)
                    .run_if(in_state(GameState::InGame).and(in_state(PauseState::Unpaused))),
            );
    }
}

#[derive(Debug, Event)]
pub struct ExplosionEvent {
    pub pos: Vec3,
    pub scale: f32,
    pub damage: f32,
    pub chain: u64,
}

#[derive(Debug, Resource, AssetCollection)]
struct ExplosionAssets {
    #[asset(path = "textures/explosion_fire.png")]
    explosion_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 5))]
    explosion_atlas_layout: Handle<TextureAtlasLayout>,
}

fn explosion_collision(
    mut reader: EventReader<ExplosionEvent>,
    read_rapier_context: ReadRapierContext,
    mut writer: EventWriter<DamageEvent>,
    query: Query<&GlobalTransform, With<Health>>,
) {
    if reader.is_empty() {
        return;
    }
    let Ok(context) = read_rapier_context.single() else {
        error!("Failed to get rapier context");
        return;
    };
    let filter = QueryFilter::new().groups(CollisionGroups {
        memberships: EXPLOSION_GROUP,
        filters: ENEMY_GROUP | PLAYER_GROUP,
    });
    for ExplosionEvent {
        pos,
        scale,
        damage,
        chain,
    } in reader.read()
    {
        let radius = 2.5 * scale;
        let shape = Collider::ball(radius);
        context.intersections_with_shape(*pos, Quat::IDENTITY, &shape, filter, |entity| {
            if let Ok(global_transform) = query.get(entity) {
                let dist = global_transform.translation().distance(*pos);
                let damage = damage * (1.0 - dist / radius).max(0.0);

                info!("explosion hit entity {}, dealing {} damage", entity, damage);
                writer.write(DamageEvent {
                    entity,
                    damage,
                    chain: *chain,
                });
            }

            // return true to keep searching
            true
        });
    }
}

fn spawn_explosion_visual(
    mut commands: Commands,
    mut reader: EventReader<ExplosionEvent>,
    assets: Res<ExplosionAssets>,
    mut sprite3d_params: Sprite3dParams,
) {
    for ExplosionEvent { pos, scale, .. } in reader.read() {
        let atlas = TextureAtlas {
            layout: assets.explosion_atlas_layout.clone(),
            index: 0,
        };
        let timer = Timer::from_seconds(0.01, TimerMode::Repeating);
        let animation = AnimatedSprite3d {
            current: 0,
            frames: vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29,
            ],
            timer,
            destroy_when_finished: true,
        };
        let explosion = Sprite3dBuilder {
            image: assets.explosion_atlas_texture.clone(),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            pixels_per_metre: 128.0,
            pivot: Some(vec2(0.5, 0.25)),
            ..Default::default()
        }
        .bundle_with_atlas(&mut sprite3d_params, atlas);
        commands.spawn((
            explosion,
            animation,
            FaceCamera::default(),
            Transform::from_translation(*pos).with_scale(Vec3::splat(5.0 * scale)),
        ));
    }
}
