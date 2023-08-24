use std::time::Duration;

use crate::*;
use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_easings::EasingsPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use bevy_tweening::*;
use bevy_ui_navigation::{prelude::*, systems::InputMapping};
use leafwing_input_manager::prelude::*;
pub fn initialize_libraries(app: &mut App) {
    // ! EASING
    app.add_plugins(EasingsPlugin);
    // ! TWEENING
    app.add_plugins(TweeningPlugin);
    // ! SAVING
    app.insert_resource(PkvStore::new("NestClimb", "savedata"));
    // ! Default plugins
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(2)),
                ..Default::default()
            }),
    )
    // ! UI
    .add_plugins(DefaultNavigationPlugins)
    .add_systems(Startup, |mut input_mapping: ResMut<InputMapping>| {
        input_mapping.keyboard_navigation = true;
        input_mapping.key_action = KeyCode::Return;
        input_mapping.focus_follows_mouse = true;
    })
    // ! Leafwing inputs
    .add_plugins(InputManagerPlugin::<PlayerAction>::default())
    .add_plugins(InputManagerPlugin::<MenuAction>::default())
    // ! LDTK levels
    .add_plugins(LdtkPlugin)
    .insert_resource(LdtkSettings {
        set_clear_color: SetClearColor::No,
        level_background: LevelBackground::Nonexistent,
        ..default()
    })
    // ! Rapier physics engine
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_systems(Startup, |mut cfg: ResMut<RapierConfiguration>| {
        cfg.gravity = Vec2::Y * -250.0;
    })
    // ! Tnua platformer controls
    .add_plugins(TnuaRapier2dPlugin)
    .add_plugins(TnuaPlatformerPlugin);
}
