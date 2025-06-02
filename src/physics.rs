use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let mut rapier_configuration = RapierConfiguration::new(1.0);
        rapier_configuration.gravity = Vec2::ZERO;

        app.add_plugins(
            RapierPhysicsPlugin::<NoUserData>::default().with_custom_initialization(
                RapierContextInitialization::InitializeDefaultRapierContext {
                    rapier_configuration,
                    integration_parameters: IntegrationParameters::default(),
                },
            ),
        )
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(TimestepMode::Interpolated {
            dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 1,
        });
    }
}
