use bevy::prelude::*;

use crate::{character_controller::ReadHeading, states::GameState};

#[derive(Debug, Default, Component)]
#[require(Name::new("Main Camera Entity"), Camera3d, Projection::Perspective(PerspectiveProjection {
    ..Default::default()
}))]
pub struct MainCamera {}

fn attach_camera_to_player(
    mut commands: Commands,
    query: Query<Entity, Added<crate::player::Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_child(MainCamera::default());
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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (attach_camera_to_player, update_heading).run_if(in_state(GameState::InGame)),
        );
    }
}
