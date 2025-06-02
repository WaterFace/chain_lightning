use bevy::prelude::*;

use crate::character_controller::ReadHeading;

#[derive(Debug, Default, Component)]
#[require(Camera3d, Projection::Perspective(PerspectiveProjection {
    ..Default::default()
}))]
pub struct MainCamera {}

fn attach_camera_to_player(
    mut commands: Commands,
    query: Query<Entity, Added<crate::player::Player>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .with_child(MainCamera::default())
            .insert(Transform::default());
    }
}

fn update_heading(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&ReadHeading, Without<MainCamera>>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        let heading = player_query
            .single()
            .unwrap_or_else(|e| panic!("Failed to get single `ReadHeading`: {e}"));

        camera_transform.look_to(heading.vec3(), Vec3::Z);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attach_camera_to_player, update_heading));
    }
}
