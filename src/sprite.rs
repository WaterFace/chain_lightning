use bevy::prelude::*;
use bevy_sprite3d::prelude::*;

use crate::states::{GameState, PauseState};

#[derive(Debug, Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin).add_systems(
            Update,
            (animate_sprites, face_camera)
                .run_if(in_state(GameState::InGame).and(in_state(PauseState::Unpaused))),
        );
    }
}

#[derive(Debug, Component)]
pub struct FaceCamera {
    pub up: Dir3,
}

impl Default for FaceCamera {
    fn default() -> Self {
        FaceCamera { up: Dir3::Y }
    }
}

fn face_camera(
    mut sprite_query: Query<
        (&mut Transform, &FaceCamera, Option<&ChildOf>),
        Without<crate::camera::MainCamera>,
    >,
    camera_transform: Single<&GlobalTransform, With<crate::camera::MainCamera>>,
    global_transforms: Query<&GlobalTransform>,
) {
    for (mut transform, face_camera, child_of) in sprite_query.iter_mut() {
        // If the object to be rotated has a parent...
        let local_transform = if let Some(&ChildOf(parent)) = child_of {
            // so long as that parent has a transform, translate the camera's
            // transformation into the object's parent's coordinate space
            if let Ok(parent_transform) = global_transforms.get(parent) {
                camera_transform.reparented_to(parent_transform)
            } else {
                // otherwise just take the camera's transformation
                camera_transform.compute_transform()
            }
        } else {
            // ditto
            camera_transform.compute_transform()
        };
        let mut delta = local_transform.translation - transform.translation;
        delta -= delta.project_onto(face_camera.up.into());
        delta += transform.translation;
        transform.look_at(delta, face_camera.up);
    }
}

#[derive(Debug, Component)]
pub struct AnimatedSprite3d {
    pub frames: Vec<usize>,
    pub current: usize,
    pub timer: Timer,
    pub destroy_when_finished: bool,
}

fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimatedSprite3d, &mut Sprite3d)>,
) {
    for (entity, mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let Some(ref mut atlas) = sprite.texture_atlas else {
                warn!("AnimatedSprite3d on an entity whose Sprite3d doesn't have a texture atlas");
                continue;
            };

            atlas.index = animation.frames[animation.current];
            animation.current += 1;
            if animation.destroy_when_finished && animation.current == animation.frames.len() {
                if let Ok(mut c) = commands.get_entity(entity) {
                    c.despawn();
                }
            }
            animation.current %= animation.frames.len();
        }
    }
}
