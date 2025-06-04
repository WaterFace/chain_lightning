use bevy::{prelude::*, render::view::RenderLayers};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_sprite3d::prelude::*;

use crate::states::{AssetLoadingExt, GameState};

#[derive(Debug, Default)]
pub struct ShotgunPlugin;

impl Plugin for ShotgunPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<ShotgunAssets>()
            .add_systems(OnEnter(GameState::InGame), setup_view_model);
    }
}

#[derive(AssetCollection, Resource, Debug)]
struct ShotgunAssets {
    #[asset(path = "shotgun_atlas.png")]
    shotgun_atlas_texture: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 384, tile_size_y = 216, columns = 1, rows = 3))]
    shotgun_atlas_layout: Handle<TextureAtlasLayout>,
}

// use render layer 1 for view model stuff
#[derive(Debug, Default, Component)]
#[require(Camera3d, Camera { order: 1, ..Default::default() }, RenderLayers::layer(1), Projection::Orthographic(OrthographicProjection {
    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 9.0 },
    ..OrthographicProjection::default_3d()
}))]
struct ViewmodelCamera;

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
    let shotgun = Sprite3dBuilder {
        image: shotgun_assets.shotgun_atlas_texture.clone(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        pixels_per_metre: 24.0,
        ..Default::default()
    }
    .bundle_with_atlas(&mut sprite3d_params, atlas);
    commands.spawn((
        shotgun,
        RenderLayers::layer(1),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}
