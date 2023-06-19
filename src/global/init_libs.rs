use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;
pub fn initialize_libraries(app: &mut App) {
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
            }),
    )
    // ! Nine Slice
    // ! Leafwing inputs
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
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
