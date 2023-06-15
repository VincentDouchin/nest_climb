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
    app.insert_resource(ExampleUiTnuaActive(true));
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
    app.add_system(track_player.run_if(run_debug));
}

// ! UI

fn track_player(mut command: Commands, player_query: Query<Entity, Added<Player>>) {
    player_query.for_each(|player_entity| {
        command
            .entity(player_entity)
            .insert(TrackedEntity("Player".to_owned()))
            .insert(PlotSource::default());
    })
}

use std::ops::RangeInclusive;

use bevy::ecs::system::EntityCommands;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_tnua::{TnuaFreeFallBehavior, TnuaPlatformerConfig};

pub struct ExampleUi;

// NOTE: The examples are responsible for taking this into account
#[derive(Resource)]
pub struct ExampleUiTnuaActive(pub bool);

#[derive(Component)]
pub struct TrackedEntity(pub String);

#[derive(Component)]
pub struct CommandAlteringSelectors(Vec<CommandAlteringSelector>);

impl Default for CommandAlteringSelectors {
    fn default() -> Self {
        Self(Default::default())
    }
}

enum CommandAlteringSelector {
    Combo {
        chosen: usize,
        caption: String,
        options: Vec<(String, fn(EntityCommands))>,
        set_to: Option<usize>,
    },
    Checkbox {
        checked: bool,
        caption: String,
        applier: fn(EntityCommands, bool),
        set_to: Option<bool>,
    },
}

impl CommandAlteringSelectors {
    pub fn with_combo(
        mut self,
        caption: &str,
        initial: usize,
        options: &[(&str, fn(EntityCommands))],
    ) -> Self {
        self.0.push(CommandAlteringSelector::Combo {
            chosen: 0,
            caption: caption.to_owned(),
            options: options
                .into_iter()
                .map(|(name, applier)| (name.to_string(), *applier))
                .collect(),
            set_to: Some(initial),
        });
        self
    }

    pub fn with_checkbox(
        mut self,
        caption: &str,
        initial: bool,
        applier: fn(EntityCommands, bool),
    ) -> Self {
        self.0.push(CommandAlteringSelector::Checkbox {
            checked: false,
            caption: caption.to_owned(),
            applier,
            set_to: Some(initial),
        });
        self
    }
}

fn slider_or_infinity(
    ui: &mut egui::Ui,
    caption: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) {
    #[derive(Clone)]
    struct CachedValue(f32);

    ui.horizontal(|ui| {
        let mut infinite = !value.is_finite();
        let resp = ui.toggle_value(&mut infinite, "\u{221e}");
        if resp.clicked() {
            if infinite {
                ui.memory_mut(|memory| memory.data.insert_temp(resp.id, CachedValue(*value)));
                *value = f32::INFINITY
            } else {
                if let Some(CachedValue(saved_value)) =
                    ui.memory_mut(|memory| memory.data.get_temp(resp.id))
                {
                    *value = saved_value;
                } else {
                    *value = *range.end();
                }
            }
        }
        if infinite {
            let mut copied_saved_value = ui.memory_mut(|memory| {
                let CachedValue(saved_value) = memory
                    .data
                    .get_temp_mut_or(resp.id, CachedValue(*range.end()));
                *saved_value
            });
            ui.add_enabled(
                false,
                egui::Slider::new(&mut copied_saved_value, range).text(caption),
            );
        } else {
            ui.add(egui::Slider::new(value, range).text(caption));
        }
    });
}

fn slider_or_none(
    ui: &mut egui::Ui,
    caption: &str,
    value: &mut Option<f32>,
    range: RangeInclusive<f32>,
) {
    #[derive(Clone)]
    struct CachedValue(f32);

    ui.horizontal(|ui| {
        let mut is_none = value.is_none();
        let resp = ui.toggle_value(&mut is_none, "\u{d8}");
        if resp.clicked() {
            if is_none {
                ui.memory_mut(|memory| memory.data.insert_temp(resp.id, CachedValue(value.expect("checkbox was clicked, and is_none is now true, so previously it was false, which means value should not be None"))));
                *value = None;
            } else {
                if let Some(CachedValue(saved_value)) =
                    ui.memory_mut(|memory| memory.data.get_temp(resp.id))
                {
                    *value = Some(saved_value);
                } else {
                    *value = Some(*range.start());
                }
            }
        }
        if let Some(value) = value.as_mut() {
            ui.add(egui::Slider::new(value, range).text(caption));
        } else {
            let mut copied_saved_value = ui.memory_mut(|memory| {
                let CachedValue(saved_value) = memory
                    .data
                    .get_temp_mut_or(resp.id, CachedValue(*range.start()));
                *saved_value
            });
            ui.add_enabled(
                false,
                egui::Slider::new(&mut copied_saved_value, range).text(caption),
            );
        }
    });
}

fn ui_system(
    mut egui_context: EguiContexts,
    mut query: Query<(
        Entity,
        &TrackedEntity,
        &PlotSource,
        &mut TnuaPlatformerConfig,
        Option<&mut CommandAlteringSelectors>,
    )>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut player_query: Query<(&mut Health, &mut CameraTarget), With<Player>>,
    mut commands: Commands,
    mut debug: ResMut<Debug>,
    mut is_touch_device: ResMut<IsTouchDevice>,
) {
    for (entity, _, _, _, command_altering_selectors) in query.iter_mut() {
        if let Some(mut command_altering_selectors) = command_altering_selectors {
            for selector in command_altering_selectors.0.iter_mut() {
                match selector {
                    CommandAlteringSelector::Combo {
                        chosen,
                        caption: _,
                        options,
                        set_to,
                    } => {
                        if let Some(set_to) = set_to.take() {
                            *chosen = set_to;
                            options[set_to].1(commands.entity(entity));
                        }
                    }
                    CommandAlteringSelector::Checkbox {
                        checked,
                        caption: _,
                        applier,
                        set_to,
                    } => {
                        if let Some(set_to) = set_to.take() {
                            *checked = set_to;
                            applier(commands.entity(entity), set_to);
                        }
                    }
                }
            }
        }
    }

    egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        if let Ok((
            entity,
            TrackedEntity(_name),
            plot_source,
            mut platformer_config,
            command_altering_selectors,
        )) = query.get_single_mut()
        {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // ! CAMERA
                    if let Ok(mut projection) = camera_query.get_single_mut() {
                        ui.add(egui::Slider::new(&mut projection.scale, 0.0..=10.0).text("Zoom"));
                    }
                    // ! PLAYER
                    if let Ok((mut player_health, mut camera_target)) =
                        player_query.get_single_mut()
                    {
                        ui.add(
                            egui::Slider::new(&mut player_health.max_health, 0..=10)
                                .text("Max Health"),
                        );
                        ui.add(
                            egui::Slider::new(&mut player_health.current_health, 0..=10)
                                .text("Current Health"),
                        );
                        ui.add(egui::Checkbox::new(&mut camera_target.x, "Camera follow X"));
                        ui.add(egui::Checkbox::new(&mut camera_target.y, "Camera follow Y"));
                    }
                    ui.add(egui::Checkbox::new(
                        &mut debug.skip_start_screen,
                        "Skip start screen",
                    ));
                    ui.add(egui::Checkbox::new(&mut is_touch_device.0, "touch device"));
                    ui.add(
                        egui::Slider::new(&mut rapier_config.gravity.y, 0.0..=-500.0)
                            .text("Gravity"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.full_speed, 0.0..=600.0)
                            .text("Speed"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.full_jump_height, 0.0..=400.0)
                            .text("Jump Height"),
                    );
                    platformer_config.full_jump_height =
                        platformer_config.full_jump_height.max(0.1);

                    if let Some(mut command_altering_selectors) = command_altering_selectors {
                        for selector in command_altering_selectors.0.iter_mut() {
                            match selector {
                                CommandAlteringSelector::Combo {
                                    chosen,
                                    caption,
                                    options,
                                    set_to: _,
                                } => {
                                    let mut selected_idx: usize = *chosen;
                                    egui::ComboBox::from_label(caption.as_str())
                                        .selected_text(&options[*chosen].0)
                                        .show_ui(ui, |ui| {
                                            for (idx, (name, _)) in options.iter().enumerate() {
                                                ui.selectable_value(&mut selected_idx, idx, name);
                                            }
                                        });
                                    if selected_idx != *chosen {
                                        options[selected_idx].1(commands.entity(entity));
                                        *chosen = selected_idx;
                                    }
                                }
                                CommandAlteringSelector::Checkbox {
                                    checked,
                                    caption,
                                    applier,
                                    set_to: _,
                                } => {
                                    if ui.checkbox(checked, caption.as_str()).clicked() {
                                        applier(commands.entity(entity), *checked);
                                    }
                                }
                            }
                        }
                    }

                    ui.add(
                        egui::Slider::new(&mut platformer_config.float_height, 0.0..=10.0)
                            .text("Float At"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.cling_distance, 0.0..=10.0)
                            .text("Cling Distance"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.spring_strengh, 0.0..=4000.0)
                            .text("Spring Strengh"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.spring_dampening, 0.0..=1.9)
                            .text("Spring Dampening"),
                    );
                    slider_or_infinity(
                        ui,
                        "Acceleration",
                        &mut platformer_config.acceleration,
                        0.0..=200.0,
                    );
                    slider_or_infinity(
                        ui,
                        "Air Acceleration",
                        &mut platformer_config.air_acceleration,
                        0.0..=200.0,
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.coyote_time, 0.0..=1.0)
                            .text("Coyote Time"),
                    );
                    ui.add(
                        egui::Slider::new(&mut platformer_config.jump_input_buffer_time, 0.0..=1.0)
                            .text("Jump Input Buffer Time"),
                    );
                    slider_or_none(
                        ui,
                        "Held Jump Cooldown",
                        &mut platformer_config.held_jump_cooldown,
                        0.0..=2.0,
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_takeoff_extra_gravity,
                            0.0..=900.0,
                        )
                        .text("Jump Takeoff Extra Gravity"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_takeoff_above_velocity,
                            0.0..=900.0,
                        )
                        .text("Jump Takeoff Above Gravity"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.height_change_impulse_for_duration,
                            0.0..=900.0,
                        )
                        .text("Height Change Impulse For Duration"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.height_change_impulse_limit,
                            0.0..=900.0,
                        )
                        .text("height Change Impulse Limit"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_fall_extra_gravity,
                            0.0..=900.0,
                        )
                        .text("Jump Fall Extra Gravity"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_shorten_extra_gravity,
                            0.0..=900.0,
                        )
                        .text("Jump Shorten Extra Gravity"),
                    );

                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_peak_prevention_at_upward_velocity,
                            0.0..=200.0,
                        )
                        .text("Jump Peak Prevention At Upward Velocity"),
                    );

                    ui.add(
                        egui::Slider::new(
                            &mut platformer_config.jump_peak_prevention_extra_gravity,
                            0.0..=500.0,
                        )
                        .text("Jump Peak Prevention Extra Gravity"),
                    );

                    let free_fall_options: [(bool, &str, fn() -> TnuaFreeFallBehavior); 3] = [
                        (
                            matches!(
                                platformer_config.free_fall_behavior,
                                TnuaFreeFallBehavior::ExtraGravity(_)
                            ),
                            "Extra Gravity",
                            || TnuaFreeFallBehavior::ExtraGravity(0.0),
                        ),
                        (
                            matches!(
                                platformer_config.free_fall_behavior,
                                TnuaFreeFallBehavior::LikeJumpShorten
                            ),
                            "Like Jump Shorten",
                            || TnuaFreeFallBehavior::LikeJumpShorten,
                        ),
                        (
                            matches!(
                                platformer_config.free_fall_behavior,
                                TnuaFreeFallBehavior::LikeJumpFall
                            ),
                            "Like Jump Fall",
                            || TnuaFreeFallBehavior::LikeJumpFall,
                        ),
                    ];
                    egui::ComboBox::from_label("Free Fall Behavior")
                        .selected_text(
                            free_fall_options
                                .iter()
                                .find_map(|(chosen, name, _)| chosen.then_some(*name))
                                .unwrap_or("???"),
                        )
                        .show_ui(ui, |ui| {
                            for (chosen, name, make_variant) in free_fall_options {
                                if ui.selectable_label(chosen, name).clicked() {
                                    platformer_config.free_fall_behavior = make_variant();
                                }
                            }
                        });
                    if let TnuaFreeFallBehavior::ExtraGravity(extra_gravity) =
                        &mut platformer_config.free_fall_behavior
                    {
                        ui.add(egui::Slider::new(extra_gravity, 0.0..=100.0).text("Extra Gravity"));
                    }

                    slider_or_infinity(
                        ui,
                        "Staying Upward Max Angular Velocity",
                        &mut platformer_config.tilt_offset_angvel,
                        0.0..=20.0,
                    );
                    slider_or_infinity(
                        ui,
                        "Staying Upward Max Angular Acceleration",
                        &mut platformer_config.tilt_offset_angacl,
                        0.0..=2000.0,
                    );

                    slider_or_infinity(
                        ui,
                        "Turning Angular Velocity",
                        &mut platformer_config.turning_angvel,
                        0.0..=70.0,
                    );
                });
                ui.vertical(|ui| {
                    plot_source.show(entity, ui);
                });
            });
        }
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
