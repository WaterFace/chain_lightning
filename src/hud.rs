use std::fmt::Write;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use num_format::{Locale, WriteFormatted};

use crate::{
    health::Health,
    player::Player,
    score::Score,
    states::{AssetLoadingExt, GameState},
};

#[derive(Debug, Default)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.load_asset_on_startup::<HudAssets>()
            .add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(Update, (update_health_display, update_score_display));
    }
}

#[derive(Resource, AssetCollection)]
struct HudAssets {
    #[asset(path = "fonts/Bore Blasters 21.ttf")]
    font: Handle<Font>,
}

#[derive(Debug, Default, Component)]
#[require(
    Camera2d,
    Camera {
        order: 2,
        ..Default::default()
    },)]
struct UiCamera;

#[derive(Debug, Default, Component)]
struct HealthDisplay;
#[derive(Debug, Default, Component)]
struct ScoreDisplay;

fn setup_hud(mut commands: Commands, assets: Res<HudAssets>) {
    commands.spawn(UiCamera);

    const FONT_SIZE: f32 = 40.0;
    commands.spawn((
        HealthDisplay,
        Text::new("+ 100"),
        TextFont {
            font: assets.font.clone(),
            font_size: FONT_SIZE,
            ..Default::default()
        },
        TextShadow::default(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.0),
            bottom: Val::Percent(5.0),
            ..Default::default()
        },
        StateScoped(GameState::InGame),
    ));

    commands.spawn((
        ScoreDisplay,
        Text::new("Score: 0"),
        TextFont {
            font: assets.font.clone(),
            font_size: FONT_SIZE,
            ..Default::default()
        },
        TextShadow::default(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.0),
            top: Val::Percent(5.0),
            ..Default::default()
        },
        StateScoped(GameState::InGame),
    ));
}

fn update_health_display(
    mut hud_query: Query<&mut Text, With<HealthDisplay>>,
    player_query: Option<Single<&Health, (With<Player>, Changed<Health>)>>,
) {
    let Some(player_health) = player_query.map(|h| h.current) else {
        return;
    };

    let player_health = (player_health.ceil() as i32).max(0);

    for mut text in hud_query.iter_mut() {
        let buf = &mut text.0;
        buf.clear();
        let _ = buf.write_str("+ ");
        let _ = buf.write_formatted(&player_health, &Locale::en);
    }
}

fn update_score_display(mut hud_query: Query<&mut Text, With<ScoreDisplay>>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }

    let score = score.score;

    for mut text in hud_query.iter_mut() {
        let buf = &mut text.0;
        buf.clear();
        let _ = buf.write_str("Score: ");
        let _ = buf.write_formatted(&score, &Locale::en);
    }
}
