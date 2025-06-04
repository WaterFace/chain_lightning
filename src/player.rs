use bevy::prelude::*;
use bevy_seedling::prelude::*;

use crate::{character_controller::CharacterController, shotgun::Shotgun};

#[derive(Debug, Default, Component)]
#[require(
    Name::new("Player"),
    Visibility::Visible,
    SpatialListener3D,
    CharacterController = CharacterController {
            max_speed: 15.0,
            acceleration: 10.0,
        },
    Shotgun,
)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}
