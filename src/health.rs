use bevy::prelude::*;

use crate::{explosion::ExplosionEvent, fire_skull::FireSkull, player::Player, states::GameState};

#[derive(Debug, Default)]
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(Update, handle_damage.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Component)]
pub struct Health {
    pub current: f32,
    pub dead: bool,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Health {
            current: health,
            dead: health <= 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
}

fn handle_damage(
    mut commands: Commands,
    mut reader: EventReader<DamageEvent>,
    mut writer: EventWriter<ExplosionEvent>,
    mut query: Query<(
        &mut Health,
        &GlobalTransform,
        Option<&Player>,
        Option<&FireSkull>,
    )>,
) {
    for DamageEvent { entity, damage } in reader.read() {
        let Ok((mut health, global_transform, player, skull)) = query.get_mut(*entity) else {
            warn!("Couldn't get Health corresponding to entity {:?}", entity);
            continue;
        };

        health.current -= damage;

        if health.current <= 0.0 && !health.dead {
            health.dead = true;
            if player.is_some() {
                info!("player died, do something");
            }
            if skull.is_some() {
                if let Ok(mut c) = commands.get_entity(*entity) {
                    c.despawn();
                    writer.write(ExplosionEvent {
                        pos: global_transform.translation(),
                        scale: 1.0,
                        damage: 25.0,
                    });
                }
            }
        }
    }
}
