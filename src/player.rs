use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionGroups;
use bevy_seedling::prelude::*;

use crate::{
    character_controller::CharacterController,
    health::Health,
    physics::{ENEMY_GROUP, EXPLOSION_GROUP, PLAYER_GROUP, WALL_GROUP},
    shotgun::Shotgun,
    states::GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHurtEvent>()
            .add_systems(Update, update_player.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Component)]
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
pub struct Player {
    pub invulnerability_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.2, TimerMode::Once);
        // start the timer in a "finished" state
        timer.tick(Duration::from_secs_f32(1000.0));
        Player {
            invulnerability_timer: timer,
        }
    }
}

impl Player {
    pub fn is_vulnerable(&self) -> bool {
        self.invulnerability_timer.finished()
    }
}

fn update_player(time: Res<Time>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        player.invulnerability_timer.tick(time.delta());
    }
}

#[derive(Debug, Event)]
pub struct PlayerHurtEvent {}
