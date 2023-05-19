use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;
pub fn initialize_libraries(app: &mut App) {
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
    );
    // ! Leafwing inputs
    app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
    // ! LDTK levels
    app.add_plugin(LdtkPlugin);
    // ! Rapier physics engine
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    app.add_startup_system(|mut cfg: ResMut<RapierConfiguration>| {
        cfg.gravity = Vec2::Y * -250.0;
    });
    // ! Tnua platformer controls
    app.add_plugin(TnuaRapier2dPlugin);
    app.add_plugin(TnuaPlatformerPlugin);
}
