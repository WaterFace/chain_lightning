use bevy::{core_pipeline::Skybox, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{
    character_controller::ReadHeading,
    states::{AppState, AssetLoadingExt, GameState},
};

#[derive(Debug, Default, Component)]
#[require(Name::new("Main Camera Entity"), Camera3d, Projection::Perspective(PerspectiveProjection {
    ..Default::default()
}))]
pub struct MainCamera {}

fn attach_camera_to_player(
    mut commands: Commands,
    query: Query<Entity, Added<crate::player::Player>>,
    assets: Res<SkyboxAssets>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_child((
            MainCamera::default(),
            Skybox {
                image: assets.skybox.clone(),
                brightness: 1000.0,
                ..Default::default()
            },
        ));
    }
}

fn update_heading(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&ReadHeading, (With<crate::player::Player>, Without<MainCamera>)>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        let heading = player_query
            .single()
            .unwrap_or_else(|e| panic!("Failed to get single `ReadHeading`: {e}"));

        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, heading.heading);
    }
}

fn fix_skybox_image(assets: Res<SkyboxAssets>, mut images: ResMut<Assets<Image>>) {
    let image = images.get_mut(&assets.skybox).unwrap();

    image.reinterpret_stacked_2d_as_array(image.height() / image.width());
    image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
        dimension: Some(bevy::render::render_resource::TextureViewDimension::Cube),
        ..Default::default()
    });
}

#[derive(AssetCollection, Resource, Debug)]
struct SkyboxAssets {
    #[asset(path = "textures/skybox.png")]
    skybox: Handle<Image>,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<SkyboxAssets>()
            .add_systems(
                OnTransition {
                    exited: AppState::AssetLoading,
                    entered: AppState::Ready,
                },
                fix_skybox_image,
            )
            .add_systems(
                Update,
                (attach_camera_to_player, update_heading).run_if(in_state(GameState::InGame)),
            );
    }
}
