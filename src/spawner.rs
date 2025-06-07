use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};

use crate::{
    assets::AssetLoadingExt,
    fire_skull::FireSkull,
    player::Player,
    sprite::{AnimatedSprite3d, FaceCamera},
    states::{GameState, PauseState},
};

#[derive(Debug, Default)]
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnParameters>()
            .add_state_scoped_event::<CreateSpawnerEvent>(GameState::InGame)
            .load_asset_on_startup::<SpawnerAssets>()
            .init_resource::<SkullsKilled>()
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    reset_skulls_killed,
                    reset_spawn_parameters,
                    create_first_spawner,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (spawn_spawners, run_spawners, create_spawners)
                    .run_if(in_state(GameState::InGame).and(in_state(PauseState::Unpaused))),
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct SkullsKilled {
    pub count: usize,
}

fn reset_skulls_killed(mut commands: Commands) {
    commands.insert_resource(SkullsKilled::default());
}

#[derive(Debug, Resource)]
struct SpawnParameters {
    skulls_to_spawn: f32,
    delay_before_next_spawner: f32,
    timer: Timer,
    difficulty: f32,
}

impl Default for SpawnParameters {
    fn default() -> Self {
        SpawnParameters {
            skulls_to_spawn: 5.0,
            delay_before_next_spawner: 10.0,
            timer: Timer::from_seconds(10.0, TimerMode::Once),
            difficulty: 0.0,
        }
    }
}

impl SpawnParameters {
    fn set_difficulty(&mut self, difficulty: f32) {
        self.difficulty = difficulty;

        self.skulls_to_spawn = 5.0 + 0.4 * f32::sqrt(30.0 * difficulty);
        self.delay_before_next_spawner = 10.0 - (5.0 * difficulty / (difficulty + 100.0));
    }
}

fn reset_spawn_parameters(mut commands: Commands) {
    commands.insert_resource(SpawnParameters::default());
}

#[derive(Debug, Component)]
struct Spawner {
    skulls_left: usize,
    timer: Timer,
}

impl Spawner {
    pub fn new(skulls_left: usize) -> Self {
        Spawner {
            skulls_left,
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
        }
    }
}

fn create_spawners(
    time: Res<Time>,
    mut writer: EventWriter<CreateSpawnerEvent>,
    mut spawn_parameters: ResMut<SpawnParameters>,
    player_query: Single<&GlobalTransform, With<Player>>,
    kill_count: Res<SkullsKilled>,
    mut rng: GlobalEntropy<WyRand>,
) {
    spawn_parameters.timer.tick(time.delta());
    if !spawn_parameters.timer.just_finished() {
        return;
    }

    let new_difficulty = kill_count.count as f32;
    spawn_parameters.set_difficulty(new_difficulty);
    let new_delay = spawn_parameters.delay_before_next_spawner;
    spawn_parameters
        .timer
        .set_duration(Duration::from_secs_f32(new_delay));
    spawn_parameters.timer.reset();

    let player_pos = player_query.translation();
    let spawn_area = Circle::new(50.0);
    let spawn_pos = loop {
        let pos = spawn_area.sample_interior(rng.as_mut());
        let pos = Vec3::new(pos.x, 0.0, pos.y);
        const FAR_ENOUGH: f32 = 15.0;
        if player_pos.distance(pos) >= FAR_ENOUGH {
            break pos;
        }
    };

    writer.write(CreateSpawnerEvent {
        pos: spawn_pos,
        skulls_left: spawn_parameters.skulls_to_spawn as usize,
    });
}

fn create_first_spawner(
    mut writer: EventWriter<CreateSpawnerEvent>,
    spawn_parameters: Res<SpawnParameters>,
) {
    writer.write(CreateSpawnerEvent {
        pos: Vec3::new(0.0, 0.0, -20.0),
        skulls_left: spawn_parameters.skulls_to_spawn as usize,
    });
}

fn run_spawners(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Spawner, &GlobalTransform)>,
) {
    for (entity, mut spawner, global_transform) in query.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.just_finished() {
            let pos = global_transform.translation();

            spawner.skulls_left -= 1;

            if spawner.skulls_left == 0 {
                if let Ok(mut c) = commands.get_entity(entity) {
                    c.despawn();
                }
            }

            commands.spawn((
                FireSkull::default(),
                Transform::from_translation(pos),
                StateScoped(GameState::InGame),
            ));
        }
    }
}

#[derive(Debug, Event)]
pub struct CreateSpawnerEvent {
    pub pos: Vec3,
    pub skulls_left: usize,
}

#[derive(Debug, Resource, AssetCollection)]
struct SpawnerAssets {
    #[asset(path = "textures/explosion_magic.png")]
    explosion_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 5))]
    explosion_atlas_layout: Handle<TextureAtlasLayout>,
}

fn spawn_spawners(
    mut commands: Commands,
    mut reader: EventReader<CreateSpawnerEvent>,
    assets: Res<SpawnerAssets>,
    mut sprite3d_params: Sprite3dParams,
) {
    for CreateSpawnerEvent { pos, skulls_left } in reader.read() {
        let atlas = TextureAtlas {
            layout: assets.explosion_atlas_layout.clone(),
            index: 0,
        };
        let timer = Timer::from_seconds(0.1, TimerMode::Repeating);
        let animation = AnimatedSprite3d {
            current: 0,
            frames: vec![19, 20, 21, 22, 23, 24],
            timer,
            destroy_when_finished: false,
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
            Spawner::new(*skulls_left),
            explosion,
            animation,
            FaceCamera::default(),
            Transform::from_translation(*pos).with_scale(Vec3::splat(8.0)),
            StateScoped(GameState::InGame),
        ));
    }
}
