use crate::*;
use bevy::prelude::*;
use bevy_egui::egui::plot::{Corner, Legend, Plot};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use std::collections::VecDeque;
pub fn debug_rendering(mut debug_config: ResMut<DebugRenderContext>, debug: Res<Debug>) {
    debug_config.enabled = debug.enabled
}
pub fn run_debug(debug: Res<Debug>) -> bool {
    debug.enabled
}

pub fn toggle_debug(keys: Res<Input<KeyCode>>, mut debug: ResMut<Debug>) {
    if keys.just_pressed(KeyCode::F1) {
        debug.enabled = !debug.enabled
    }
}

#[derive(Resource)]
pub struct Debug {
    pub enabled: bool,
    pub skip_start_screen: bool,
}
pub fn save_debug(debug: Res<Debug>, mut pkv: ResMut<PkvStore>) {
    if debug.is_changed() {
        pkv.set("skip start screen", &debug.skip_start_screen)
            .expect("save skip start screen");
    }
}
pub fn set_debug(mut debug: ResMut<Debug>, pkv: Res<PkvStore>) {
    if let Ok(skip_start_screen) = pkv.get("skip start screen") {
        debug.skip_start_screen = skip_start_screen
    }
}
pub fn skip_start_screen(
    debug: Res<Debug>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    if debug.skip_start_screen {
        commands.insert_resource(CurrentLevel {
            file: Some(assets.test_level.clone()),
        });
        next_state.set(GameState::Run);
    }
}
pub fn debug_plugin(app: &mut App) {
    app.add_plugin(RapierDebugRenderPlugin::default());
    app.add_system(save_debug);
    app.insert_resource(Debug {
        enabled: false,
        skip_start_screen: false,
    });
    app.add_startup_system(set_debug);
    app.add_system(skip_start_screen.in_schedule(OnEnter(GameState::Start)));
    app.add_plugin(EguiPlugin);
    app.add_system(toggle_debug);
    app.add_system(debug_rendering);
    app.add_system(ui_system.run_if(run_debug));
    app.add_system(plot_source_rolling_update.run_if(run_debug));
}

use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn ui_system(
    mut egui_context: EguiContexts,

    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut player_query: Query<(&mut Health, &mut CameraTarget, &mut MovementControl), With<Player>>,
    mut debug: ResMut<Debug>,
    mut is_touch_device: ResMut<IsTouchDevice>,
) {
    egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                // ! CAMERA
                if let Ok(mut projection) = camera_query.get_single_mut() {
                    ui.add(egui::Slider::new(&mut projection.scale, 0.0..=10.0).text("Zoom"));
                }
                // ! PLAYER
                if let Ok((mut player_health, mut camera_target, mut controls)) =
                    player_query.get_single_mut()
                {
                    ui.add(
                        egui::Slider::new(&mut player_health.max_health, 0..=10).text("Max Health"),
                    );
                    ui.add(
                        egui::Slider::new(&mut player_health.current_health, 0..=10)
                            .text("Current Health"),
                    );
                    ui.add(egui::Checkbox::new(&mut camera_target.x, "Camera follow X"));
                    ui.add(egui::Checkbox::new(&mut camera_target.y, "Camera follow Y"));
                    ui.add(egui::Slider::new(&mut controls.speed, 0.0..=500.0).text("speed"));
                    ui.add(
                        egui::Slider::new(&mut controls.acceleration, 0.0..=500.0)
                            .text("acceleration"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.deceleration, 0.0..=500.0)
                            .text("deceleration"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.air_acceleration, 0.0..=500.0)
                            .text("air acceleration"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.air_deceleration, 0.0..=500.0)
                            .text("air deceleration"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.turn_speed, 0.0..=2000.0)
                            .text("turn speed"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.jump_height, 0.0..=2000.0)
                            .text("jump height"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.max_fall_speed, 0.0..=2000.0)
                            .text("max_fall_speed"),
                    );

                    ui.add(
                        egui::Slider::new(&mut controls.upward_multiplier, 0.0..=10.0)
                            .text("upward_multiplier"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.downward_multiplier, 0.0..=10.0)
                            .text("downward_multiplier"),
                    );
                    ui.add(
                        egui::Slider::new(&mut controls.jump_cut_off, 0.0..=10.0)
                            .text("jump_cut_off"),
                    );
                }
                ui.add(egui::Checkbox::new(
                    &mut debug.skip_start_screen,
                    "Skip start screen",
                ));
                ui.add(egui::Checkbox::new(&mut is_touch_device.0, "touch device"));
                ui.add(
                    egui::Slider::new(&mut rapier_config.gravity.y, 0.0..=-500.0).text("Gravity"),
                );
            })
        });
    });
}

#[derive(Component, Debug)]
pub struct PlotSource {
    input: Vec<Vec<(&'static str, f32)>>,
    fields: Vec<Vec<&'static str>>,
    rolling: VecDeque<f32>,
    last_update: f32,
    update_every: f32,
    keep: f32,
}

impl Default for PlotSource {
    fn default() -> Self {
        Self {
            input: Default::default(),
            fields: Default::default(),
            rolling: Default::default(),
            last_update: f32::NEG_INFINITY,
            update_every: 1.0 / 24.0,
            keep: 5.0,
        }
    }
}

impl PlotSource {
    pub fn set(&mut self, input: &[&[(&'static str, f32)]]) {
        if self.input.is_empty() {
            self.input = input
                .iter()
                .map(|plot| plot.iter().map(|curve_data| *curve_data).collect())
                .collect();
        } else {
            for (target_plot, source_plot) in self.input.iter_mut().zip(input) {
                for (target_curve, source_curve) in target_plot.iter_mut().zip(*source_plot) {
                    *target_curve = *source_curve;
                }
            }
        }
    }

    pub fn show(&self, entity: Entity, ui: &mut egui::Ui) {
        let mut plots_data = self
            .fields
            .iter()
            .map(|plot| {
                plot.iter()
                    .map(|_| Vec::<[f64; 2]>::new())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut it = self.rolling.iter();
        while let Some(timestamp) = it.next() {
            for plot_data in plots_data.iter_mut() {
                for curve in plot_data.iter_mut() {
                    curve.push([*timestamp as f64, *it.next().unwrap() as f64]);
                }
            }
        }
        for (i, (plot_fields, plot_data)) in self.fields.iter().zip(plots_data).enumerate() {
            let plot = Plot::new((entity, i))
                .legend(Legend::default().position(Corner::LeftBottom))
                .width(180.0)
                .height(180.0)
                .include_y(-20.0)
                .include_y(20.0)
                .show_axes([false, true]);
            plot.show(ui, |plot_ui| {
                for (field, curve) in plot_fields.iter().zip(plot_data) {
                    plot_ui.line(egui::plot::Line::new(curve).name(field));
                }
            });
        }
    }
}

pub fn plot_source_rolling_update(time: Res<Time>, mut query: Query<&mut PlotSource>) {
    let time = time.elapsed_seconds();
    for mut plot_source in query.iter_mut() {
        if plot_source.input.is_empty() {
            continue;
        }
        if time - plot_source.last_update < plot_source.update_every {
            continue;
        }
        let keep_from = time - plot_source.keep;
        plot_source.last_update = time;
        if plot_source.fields.is_empty() {
            plot_source.fields = plot_source
                .input
                .iter()
                .map(|plot| plot.iter().map(|(name, _)| *name).collect())
                .collect();
        }

        let record_width = 1 + plot_source
            .fields
            .iter()
            .map(|flds| flds.len())
            .sum::<usize>();
        while let Some(timestamp) = plot_source.rolling.front() {
            assert!(0 < record_width);
            if keep_from <= *timestamp {
                break;
            }
            plot_source.rolling.drain(0..record_width);
        }

        plot_source.rolling.push_back(time);
        {
            let PlotSource { input, rolling, .. } = &mut *plot_source;
            rolling.extend(
                input
                    .iter()
                    .flat_map(|plot| plot.iter().map(|(_, value)| *value)),
            );
        }
    }
}
