use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::IntegrationParameters};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let mut rapier_configuration = RapierConfiguration::new(1.0);
        rapier_configuration.gravity = Vec3::ZERO;

        app.add_plugins(
            RapierPhysicsPlugin::<NoUserData>::default().with_custom_initialization(
                RapierContextInitialization::InitializeDefaultRapierContext {
                    rapier_configuration,
                    integration_parameters: IntegrationParameters::default(),
                },
            ),
        )
        // .add_plugins(RapierDebugRenderPlugin::default())
        ;
    }
}

pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;
pub const SHOTGUN_GROUP: Group = Group::GROUP_3;
pub const EXPLOSION_GROUP: Group = Group::GROUP_4;
pub const WALL_GROUP: Group = Group::GROUP_5;
