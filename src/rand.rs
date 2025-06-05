use bevy::prelude::*;
use bevy_rand::prelude::*;

#[derive(Debug, Default)]
pub struct RandPlugin;

impl Plugin for RandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::default());
    }
}
