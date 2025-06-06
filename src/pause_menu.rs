use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiPlugin,
    egui::{self, Align2, RichText, emath::Numeric},
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    audio::AudioSettings,
    camera::CameraSettings,
    input::{InputAction, InputSettings},
    states::{GameState, PauseState},
};

#[derive(Debug, Default)]
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .init_resource::<Settings>()
        .add_systems(
            EguiContextPass,
            pause_menu.run_if(in_state(PauseState::Paused)),
        )
        .add_systems(Update, pause_unpause.run_if(in_state(GameState::InGame)))
        .add_systems(Update, update_individual_settings)
        .add_systems(OnEnter(PauseState::Paused), on_pause)
        .add_systems(OnEnter(PauseState::Unpaused), on_unpause);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Setting<T> {
    value: T,
    min: T,
    max: T,
}

impl<T> Setting<T> {
    pub fn new(value: T, min: T, max: T) -> Self {
        Self { value, min, max }
    }
}

impl<T: Numeric> Setting<T> {
    fn slider(&mut self) -> egui::Slider {
        let min = self.min;
        let max = self.max;
        egui::Slider::new(&mut self.value, min..=max)
    }
}

#[derive(Debug, Resource)]
pub struct Settings {
    // sound settings:
    sfx_volume: Setting<f32>,
    music_volume: Setting<f32>,

    // input settings:
    turn_rate: Setting<f32>,
    mouse_sensitivity: Setting<f32>,

    // camera settings:
    fov: Setting<f32>,
}

impl Default for Settings {
    fn default() -> Self {
        let default_sound_settings = AudioSettings::default();
        let default_input_settings = InputSettings::default();
        let default_camera_settings = CameraSettings::default();

        Settings {
            sfx_volume: Setting::new(default_sound_settings.sound_effect_volume, 0.0, 1.0),
            music_volume: Setting::new(default_sound_settings.music_volume, 0.0, 1.0),
            turn_rate: Setting::new(default_input_settings.turn_rate, 0.1, 3.0),
            mouse_sensitivity: Setting::new(default_input_settings.mouse_sensitivity, 0.01, 0.5),
            fov: Setting::new(default_camera_settings.fov, 30.0, 130.0),
        }
    }
}

fn update_individual_settings(
    settings: Res<Settings>,
    mut audio_settings: ResMut<AudioSettings>,
    mut input_settings: ResMut<InputSettings>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    if !settings.is_changed() {
        return;
    }

    audio_settings.sound_effect_volume = settings.sfx_volume.value;
    audio_settings.music_volume = settings.music_volume.value;

    input_settings.turn_rate = settings.turn_rate.value;
    input_settings.mouse_sensitivity = settings.mouse_sensitivity.value;

    camera_settings.fov = settings.fov.value;
}

fn pause_unpause(
    current_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    input: Res<ActionState<InputAction>>,
) {
    if !input.just_pressed(&InputAction::Pause) {
        return;
    }

    match *current_state.get() {
        PauseState::Paused => next_state.set(PauseState::Unpaused),
        PauseState::Unpaused => next_state.set(PauseState::Paused),
    }
}

fn on_pause(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.pause();
}

fn on_unpause(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.unpause();
}

fn pause_menu(
    mut contexts: EguiContexts,
    main_window: Single<&Window, With<PrimaryWindow>>,
    mut settings: ResMut<Settings>,
    mut exit_confirm: Local<bool>,
) {
    egui::Window::new("Paused")
        .auto_sized()
        .movable(false)
        .pivot(Align2::CENTER_CENTER)
        .default_pos((main_window.size() / 2.0).to_array())
        .collapsible(false)
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("settings_grid").show(ui, |ui| {
                ui.vertical_centered(|ui| ui.heading("Audio"));
                ui.end_row();

                ui.label("SFX Volume");
                ui.add(settings.sfx_volume.slider());
                ui.end_row();

                ui.label("Music Volume");
                ui.add(settings.music_volume.slider());
                ui.end_row();

                ui.vertical_centered(|ui| ui.heading("Input"));
                ui.end_row();

                ui.label("Turn Rate");
                ui.add(settings.turn_rate.slider());
                ui.end_row();

                ui.label("Mouse Sensitivity");
                ui.add(settings.mouse_sensitivity.slider());
                ui.end_row();

                ui.vertical_centered(|ui| ui.heading("Camera"));
                ui.end_row();

                ui.label("FOV");
                ui.add(settings.fov.slider());
                ui.end_row();
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if *exit_confirm {
                    if ui
                        .button(RichText::new("Are you sure?").color(egui::Color32::RED))
                        .clicked()
                    {
                        *exit_confirm = false;
                        info!("exit");
                    }
                } else if ui.button("Exit To Menu").clicked() {
                    *exit_confirm = true;
                }

                if ui.button("Resume").clicked() {
                    *exit_confirm = false;
                    info!("resume");
                }
            })
        });
}
