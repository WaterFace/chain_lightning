use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionGroups;
use bevy_seedling::prelude::*;

use crate::{
    character_controller::CharacterController,
    health::Health,
    physics::{ENEMY_GROUP, EXPLOSION_GROUP, PLAYER_GROUP, WALL_GROUP},
    shotgun::Shotgun,
};

#[derive(Debug, Default, Component)]
#[require(
    Name::new("Player"),
    Health::new(100.0),
    Visibility::Visible,
    SpatialListener3D,
    CharacterController = CharacterController {
            max_speed: 15.0,
            acceleration: 10.0,
        },
    CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP | EXPLOSION_GROUP | WALL_GROUP),
    Shotgun,
)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}
