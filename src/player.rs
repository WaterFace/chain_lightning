use bevy::prelude::*;

#[derive(Debug, Default, Component)]
#[require(crate::character_controller::CharacterController)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}
