use bevy::prelude::*;

use crate::{
    explosion::ExplosionEvent,
    fire_skull::FireSkull,
    player::{Player, PlayerHurtEvent},
    score::ScoreEvent,
    spawner::SkullsKilled,
    states::{GameState, PauseState},
};

#[derive(Debug, Default)]
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_systems(
            Update,
            handle_damage.run_if(in_state(GameState::InGame).and(in_state(PauseState::Unpaused))),
        );
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
    pub chain: u64,
}

fn handle_damage(
    mut commands: Commands,
    mut reader: EventReader<DamageEvent>,
    mut explosion_writer: EventWriter<ExplosionEvent>,
    mut player_hurt_writer: EventWriter<PlayerHurtEvent>,
    mut score_writer: EventWriter<ScoreEvent>,
    mut kill_count: ResMut<SkullsKilled>,
    mut query: Query<(
        &mut Health,
        &GlobalTransform,
        Option<&mut Player>,
        Option<&FireSkull>,
    )>,
) {
    for DamageEvent {
        entity,
        damage,
        chain,
    } in reader.read()
    {
        let Ok((mut health, global_transform, mut player, skull)) = query.get_mut(*entity) else {
            continue;
        };

        if let Some(ref mut player) = player {
            if player.is_vulnerable() && *damage > 0.0 {
                health.current -= damage;
                player.invulnerability_timer.reset();
                player_hurt_writer.write(PlayerHurtEvent {});
            }
        } else {
            health.current -= damage;
        }

        if health.current <= 0.0 && !health.dead {
            health.dead = true;
            if player.is_some() {
                info!("player died, do something");
            }
            if skull.is_some() {
                if let Ok(mut c) = commands.get_entity(*entity) {
                    c.despawn();
                    explosion_writer.write(ExplosionEvent {
                        pos: global_transform.translation(),
                        scale: 1.0,
                        damage: 25.0,
                        chain: *chain + 1,
                    });
                    kill_count.count += 1;
                    score_writer.write(ScoreEvent { chain: *chain });
                }
            }
        }
    }
}
