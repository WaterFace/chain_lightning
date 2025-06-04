use bevy::prelude::*;

use crate::character_controller::CharacterController;

#[derive(Debug, Default, Component)]
#[require(
    Name::new("Player"),
    Visibility::Visible,
    CharacterController = CharacterController {
            max_speed: 15.0,
            acceleration: 10.0,
        }
)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}
