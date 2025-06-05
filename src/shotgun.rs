use bevy::{prelude::*, render::view::RenderLayers};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    character_controller::ReadHeading,
    health::DamageEvent,
    input::PlayerAction,
    physics::{ENEMY_GROUP, SHOTGUN_GROUP},
    player::Player,
    states::{AssetLoadingExt, GameState},
};

#[derive(Debug, Default)]
pub struct ShotgunPlugin;

impl Plugin for ShotgunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShotgunEvent>()
            .load_asset_on_startup::<ShotgunAssets>()
            .add_systems(OnEnter(GameState::InGame), setup_view_model)
            .add_systems(
                Update,
                (
                    update_shotgun,
                    animate_shotgun.after(update_shotgun),
                    cast_to_hit,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

// TODO: hook up sounds
#[derive(Debug, Clone, Copy, Event, Hash, PartialEq, Eq)]
pub enum ShotgunEvent {
    Fire,
    Reload,
}

#[derive(Debug, Component)]
pub struct Shotgun {
    pub state: ShotgunState,
    pub next_state: ShotgunState,
    pub shots: usize,
    pub firing_time: f32,
    pub reloading_time: f32,
    pub falloff_start: f32,
    pub falloff_end: f32,
    pub damage: f32,
}

impl Shotgun {
    fn should_fire(&self, fire_pressed: bool) -> bool {
        if fire_pressed && matches!(self.state, ShotgunState::Idle) && self.shots == 2 {
            return true;
        }

        if !fire_pressed && matches!(self.state, ShotgunState::Idle) && self.shots == 1 {
            return true;
        }

        false
    }
}

impl Default for Shotgun {
    fn default() -> Self {
        Shotgun {
            state: ShotgunState::default(),
            next_state: ShotgunState::Idle,
            shots: 2,
            firing_time: 0.05,
            reloading_time: 1.0,
            falloff_start: 15.0,
            falloff_end: 30.0,
            damage: 100.0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ShotgunState {
    #[default]
    Idle,
    Firing {
        firing_timer: Timer,
    },
    Reloading {
        reload_timer: Timer,
    },
}

fn update_shotgun(
    time: Res<Time>,
    input: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut Shotgun>,
    mut writer: EventWriter<ShotgunEvent>,
) {
    for mut shotgun in query.iter_mut() {
        match shotgun.state {
            ShotgunState::Idle => {
                if !shotgun.should_fire(input.pressed(&PlayerAction::Fire)) {
                    continue;
                }

                shotgun.state = ShotgunState::Firing {
                    firing_timer: Timer::from_seconds(shotgun.firing_time, TimerMode::Once),
                };
                if shotgun.shots == 1 {
                    shotgun.shots = 0;
                    shotgun.next_state = ShotgunState::Reloading {
                        reload_timer: Timer::from_seconds(shotgun.reloading_time, TimerMode::Once),
                    }
                } else {
                    shotgun.shots -= 1;
                    shotgun.next_state = ShotgunState::Idle
                }
                writer.write(ShotgunEvent::Fire);
            }

            ShotgunState::Firing {
                ref mut firing_timer,
            } => {
                firing_timer.tick(time.delta());
                if firing_timer.finished() {
                    shotgun.state = shotgun.next_state.clone();

                    if matches!(shotgun.next_state, ShotgunState::Reloading { .. }) {
                        writer.write(ShotgunEvent::Reload);
                    }
                }
            }
            ShotgunState::Reloading {
                ref mut reload_timer,
            } => {
                reload_timer.tick(time.delta());
                if reload_timer.finished() {
                    shotgun.state = ShotgunState::Idle;
                    shotgun.shots = 2;
                }
            }
        }
    }
}

fn cast_to_hit(
    mut reader: EventReader<ShotgunEvent>,
    shotgun_query: Query<(&GlobalTransform, &ReadHeading, &Shotgun)>,
    read_rapier_context: ReadRapierContext,
    mut writer: EventWriter<DamageEvent>,
) {
    if !reader.read().any(|ev| matches!(ev, ShotgunEvent::Fire)) {
        return;
    }

    let Ok(context) = read_rapier_context.single() else {
        error!("Failed to get rapier context");
        return;
    };

    for (transform, heading, shotgun) in shotgun_query.iter() {
        let pos = transform.translation();
        let dir = heading.to_vec3();
        let shape = Collider::ball(0.3);
        let options = ShapeCastOptions::default();
        let filter = QueryFilter::new().groups(CollisionGroups {
            memberships: SHOTGUN_GROUP,
            filters: ENEMY_GROUP,
        });

        if let Some((entity, hit)) =
            context.cast_shape(pos, Rot::IDENTITY, dir, &shape, options, filter)
        {
            let dist = hit.time_of_impact;
            let damage = if dist <= shotgun.falloff_start {
                shotgun.damage
            } else {
                let t =
                    (dist - shotgun.falloff_start) / (shotgun.falloff_end - shotgun.falloff_start);

                f32::max(shotgun.damage * (1.0 - t), 0.0)
            };
            info!(
                "hit entity {:?} at a distance of {} for {} damage",
                entity, dist, damage
            );
            writer.write(DamageEvent { entity, damage });
        }
    }
}

fn animate_shotgun(
    shotgun_query: Query<&Shotgun, With<Player>>,
    mut view_model_query: Query<&mut Sprite3d, With<ShotgunViewModel>>,
) {
    let Ok(shotgun) = shotgun_query.single() else {
        warn!("didn't find exactly one player with shotgun");
        return;
    };
    for mut sprite in view_model_query.iter_mut() {
        let Some(ref mut atlas) = sprite.texture_atlas else {
            warn!("Shotgun sprite doesn't have a texture atlas");
            continue;
        };
        match shotgun.state {
            ShotgunState::Idle => {
                atlas.index = ShotgunAssets::IDLE;
            }
            ShotgunState::Firing { .. } => {
                atlas.index = ShotgunAssets::FIRING;
            }
            ShotgunState::Reloading { .. } => {
                atlas.index = ShotgunAssets::RELOADING;
            }
        }
    }
}

#[derive(AssetCollection, Resource, Debug)]
struct ShotgunAssets {
    #[asset(path = "textures/shotgun_atlas.png")]
    shotgun_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 384, tile_size_y = 216, columns = 1, rows = 3))]
    shotgun_atlas_layout: Handle<TextureAtlasLayout>,
}

impl ShotgunAssets {
    // frame indices
    const IDLE: usize = 0;
    const FIRING: usize = 1;
    const RELOADING: usize = 2;
}

// use render layer 1 for view model stuff
#[derive(Debug, Default, Component)]
#[require(Camera3d, Camera { order: 1, ..Default::default() }, RenderLayers::layer(1), Projection::Orthographic(OrthographicProjection {
    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 9.0 },
    ..OrthographicProjection::default_3d()
}))]
struct ViewmodelCamera;

#[derive(Debug, Default, Component)]
struct ShotgunViewModel;

fn setup_view_model(
    mut commands: Commands,
    shotgun_assets: Res<ShotgunAssets>,
    mut sprite3d_params: Sprite3dParams,
) {
    commands.spawn(ViewmodelCamera);

    let atlas = TextureAtlas {
        layout: shotgun_assets.shotgun_atlas_layout.clone(),
        index: 0,
    };
    let shotgun_sprite = Sprite3dBuilder {
        image: shotgun_assets.shotgun_atlas_texture.clone(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        pixels_per_metre: 24.0,
        ..Default::default()
    }
    .bundle_with_atlas(&mut sprite3d_params, atlas);
    commands.spawn((
        shotgun_sprite,
        ShotgunViewModel,
        RenderLayers::layer(1),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}
