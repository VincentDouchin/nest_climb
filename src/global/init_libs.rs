use crate::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_easings::EasingsPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_ninepatch::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use bevy_ui_navigation::{prelude::*, systems::InputMapping};
use leafwing_input_manager::prelude::*;
pub fn initialize_libraries(app: &mut App) {
    // ! EASING
    app.add_plugin(EasingsPlugin);
    // ! SAVING
    app.insert_resource(PkvStore::new("NestClimb", "savedata"));
    // ! Default plugins
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),

                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes: true,
                ..Default::default()
            }),
    )
    // ! NINE PATCH
    .add_plugin(NinePatchPlugin::<()>::default())
    // ! UI
    .add_plugins(DefaultNavigationPlugins)
    .add_startup_system(|mut input_mapping: ResMut<InputMapping>| {
        input_mapping.keyboard_navigation = true;
        input_mapping.key_action = KeyCode::Return;
        input_mapping.focus_follows_mouse = true;
    })
    // ! Leafwing inputs
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(InputManagerPlugin::<MenuAction>::default())
    // ! LDTK levels
    .add_plugin(LdtkPlugin)
    // ! Rapier physics engine
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_startup_system(|mut cfg: ResMut<RapierConfiguration>| {
        cfg.gravity = Vec2::Y * -250.0;
    })
    // ! Tnua platformer controls
    .add_plugin(TnuaRapier2dPlugin)
    .add_plugin(TnuaPlatformerPlugin);
}
