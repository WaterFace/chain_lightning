use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_rapier3d::prelude::{Collider, CollisionGroups};
use bevy_sprite3d::prelude::*;

use crate::{
    physics::{PLAYER_GROUP, WALL_GROUP},
    states::{AssetLoadingExt, GameState},
};

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<LevelAssets>()
            .add_systems(OnEnter(GameState::InGame), spawn_ground_and_walls);
    }
}

fn spawn_ground_and_walls(
    mut commands: Commands,
    assets: Res<LevelAssets>,
    mut sprite3d_params: Sprite3dParams,
) {
    // width of the platform is 2*HALF_WIDTH + 1 to guarantee it has odd side lengths
    const HALF_WIDTH: usize = 7;
    const WIDTH: usize = 2 * HALF_WIDTH + 1;
    const NUM_TILES: usize = WIDTH * WIDTH;
    const TILE_SIZE: f32 = 4.0;

    // ground
    for i in 0..NUM_TILES {
        let x = i % WIDTH;
        let y = i / WIDTH;

        let offset = Vec3::new(
            TILE_SIZE * (x as f32 + 0.5 - (HALF_WIDTH as f32 + 0.5)),
            -1.0,
            TILE_SIZE * (y as f32 + 0.5 - (HALF_WIDTH as f32 + 0.5)),
        );
        let index = {
            if x == 0 && y == 0 {
                LevelAssets::BOTTOM_LEFT
            } else if x == 0 && y == WIDTH - 1 {
                LevelAssets::TOP_LEFT
            } else if x == WIDTH - 1 && y == 0 {
                LevelAssets::BOTTOM_RIGHT
            } else if x == WIDTH - 1 && y == WIDTH - 1 {
                LevelAssets::TOP_RIGHT
            } else if x == 0 {
                LevelAssets::LEFT_MIDDLE
            } else if x == WIDTH - 1 {
                LevelAssets::RIGHT_MIDDLE
            } else if y == 0 {
                LevelAssets::BOTTOM_MIDDLE
            } else if y == WIDTH - 1 {
                LevelAssets::TOP_MIDDLE
            } else {
                LevelAssets::MIDDLE_MIDDLE
            }
        };
        let atlas = TextureAtlas {
            layout: assets.brick_atlas_layout.clone(),
            index,
        };
        let tile = Sprite3dBuilder {
            alpha_mode: AlphaMode::Opaque,
            pixels_per_metre: 16.0 / TILE_SIZE,
            unlit: true,
            image: assets.brick_atlas_texture.clone(),
            double_sided: true,
            ..Default::default()
        }
        .bundle_with_atlas(&mut sprite3d_params, atlas);

        commands.spawn((
            tile,
            Transform::from_translation(offset).looking_to(Dir3::Y, Dir3::Z),
        ));
    }

    // walls
    const WALL_HALF_SIZE: f32 = 0.5 * WIDTH as f32 * TILE_SIZE;
    let groups = CollisionGroups {
        memberships: WALL_GROUP,
        filters: PLAYER_GROUP,
    };
    let collider = Collider::cuboid(WALL_HALF_SIZE, 4.0, WALL_HALF_SIZE);
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x == 0 && y == 0 {
                continue;
            }

            let offset = 2.0 * WALL_HALF_SIZE * Vec3::new(x as f32, 0.0, y as f32);
            commands.spawn((
                collider.clone(),
                Transform::from_translation(offset),
                groups,
            ));
        }
    }
}

#[derive(Debug, Resource, AssetCollection)]
struct LevelAssets {
    #[asset(path = "textures/brick.png")]
    brick_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 4))]
    brick_atlas_layout: Handle<TextureAtlasLayout>,
}

impl LevelAssets {
    const TOP_LEFT: usize = LevelAssets::tile(1, 1);
    const TOP_MIDDLE: usize = LevelAssets::tile(2, 1);
    const TOP_RIGHT: usize = LevelAssets::tile(3, 1);
    const LEFT_MIDDLE: usize = LevelAssets::tile(1, 2);
    const MIDDLE_MIDDLE: usize = LevelAssets::tile(2, 2);
    const RIGHT_MIDDLE: usize = LevelAssets::tile(3, 2);
    const BOTTOM_LEFT: usize = LevelAssets::tile(1, 3);
    const BOTTOM_MIDDLE: usize = LevelAssets::tile(2, 3);
    const BOTTOM_RIGHT: usize = LevelAssets::tile(3, 3);

    const fn tile(x: usize, y: usize) -> usize {
        x + y * 12
    }
}
