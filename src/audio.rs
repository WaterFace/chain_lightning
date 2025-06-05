use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::{prelude::*, sample::Sample};

use crate::states::{AssetLoadingExt, GameState};

#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SeedlingPlugin::default())
            .init_resource::<AudioSettings>()
            .load_asset_on_startup::<SoundAssets>()
            // really do this in the Startup schedule, as this should
            // only happen once and persist through the whole application
            .add_systems(Startup, setup_pools)
            // this should run regardless of the game's state
            .add_systems(Update, update_audio_settings)
            .add_systems(
                Update,
                (
                    play_shotgun_sounds,
                    play_explosion_sounds,
                    play_spawner_sounds,
                    play_player_sounds,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Debug, Resource)]
pub struct AudioSettings {
    pub sound_effect_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            sound_effect_volume: 0.5,
        }
    }
}

fn update_audio_settings(
    settings: Res<AudioSettings>,
    sound_effect_query: Query<
        &SampleEffects,
        Or<(With<SoundEffectPool>, With<SpatialSoundEffectPool>)>,
    >,
    sound_effect_pool_query: Query<
        &SampleEffects,
        Or<(
            With<SamplerPool<SoundEffectPool>>,
            With<SamplerPool<SpatialSoundEffectPool>>,
        )>,
    >,
    mut volume_query: Query<&mut VolumeNode>,
) {
    if !settings.is_changed() {
        return;
    }

    for effects in sound_effect_query.iter() {
        match volume_query.get_effect_mut(effects) {
            Ok(mut volume) => {
                *volume = VolumeNode {
                    volume: Volume::Linear(settings.sound_effect_volume),
                }
            }
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        }
    }

    for pool in sound_effect_pool_query.iter() {
        match volume_query.get_effect_mut(pool) {
            Ok(mut volume) => {
                *volume = VolumeNode {
                    volume: Volume::Linear(settings.sound_effect_volume),
                }
            }
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        }
    }
}

#[derive(PoolLabel, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct SoundEffectPool;

#[derive(PoolLabel, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct SpatialSoundEffectPool;

fn setup_pools(mut commands: Commands, settings: Res<AudioSettings>) {
    commands.spawn((
        SamplerPool(SoundEffectPool),
        sample_effects![VolumeNode {
            volume: Volume::Linear(settings.sound_effect_volume)
        }],
    ));

    commands.spawn((
        SamplerPool(SpatialSoundEffectPool),
        sample_effects![
            VolumeNode {
                volume: Volume::Linear(settings.sound_effect_volume)
            },
            SpatialBasicNode::default()
        ],
    ));
}

#[derive(Debug, Resource, AssetCollection)]
struct SoundAssets {
    // shotgun sounds
    #[asset(path = "sounds/reload.ogg")]
    reload: Handle<Sample>,
    #[asset(path = "sounds/gunshot.ogg")]
    gunshot: Handle<Sample>,

    // explosion sounds
    #[asset(path = "sounds/explosion.ogg")]
    explosion: Handle<Sample>,

    // spawner sounds
    #[asset(path = "sounds/portal.ogg")]
    portal: Handle<Sample>,

    // player sounds
    #[asset(path = "sounds/oof.ogg")]
    oof: Handle<Sample>,
}

fn play_shotgun_sounds(
    mut commands: Commands,
    mut reader: EventReader<crate::shotgun::ShotgunEvent>,
    assets: Res<SoundAssets>,
) {
    use crate::shotgun::ShotgunEvent;
    for ev in reader.read() {
        match ev {
            ShotgunEvent::Fire => {
                commands.spawn((SamplePlayer::new(assets.gunshot.clone()), SoundEffectPool))
            }
            ShotgunEvent::Reload => {
                commands.spawn((SamplePlayer::new(assets.reload.clone()), SoundEffectPool))
            }
        };
    }
}

fn play_explosion_sounds(
    mut commands: Commands,
    mut reader: EventReader<crate::explosion::ExplosionEvent>,
    assets: Res<SoundAssets>,
) {
    for crate::explosion::ExplosionEvent { pos, .. } in reader.read() {
        commands.spawn((
            SamplePlayer::new(assets.explosion.clone()),
            SpatialSoundEffectPool,
            Transform::from_translation(*pos),
        ));
    }
}

fn play_spawner_sounds(
    mut commands: Commands,
    mut reader: EventReader<crate::spawner::CreateSpawnerEvent>,
    assets: Res<SoundAssets>,
) {
    for crate::spawner::CreateSpawnerEvent { pos, .. } in reader.read() {
        commands.spawn((
            SamplePlayer::new(assets.portal.clone()),
            SpatialSoundEffectPool,
            sample_effects!(SpatialBasicNode {
                damping_distance: -1.0,
                volume: Volume::Linear(2.0),
                ..Default::default()
            }),
            Transform::from_translation(*pos),
        ));
    }
}

fn play_player_sounds(
    mut commands: Commands,
    mut reader: EventReader<crate::player::PlayerHurtEvent>,
    assets: Res<SoundAssets>,
) {
    for crate::player::PlayerHurtEvent { .. } in reader.read() {
        commands.spawn((SamplePlayer::new(assets.oof.clone()), SoundEffectPool));
    }
}
