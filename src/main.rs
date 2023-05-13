use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use nest_climb::{
    animate_sprites, camera_plugin, debug_plugin, initialize_libraries, load_assets_plugin,
    move_player_system, spawn_ground, spawn_map, spawn_player, update_direction, GameState,
};
use seldom_fn_plugin::FnPluginExt;
fn main() {
    App::new()
        // ! Libraries
        .fn_plugin(initialize_libraries)
        // ! Debug
        .fn_plugin(debug_plugin)
        // ! States
        .add_state::<GameState>()
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // !Camera
        .fn_plugin(camera_plugin)
        // ! Spawn entities
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_ground.in_schedule(OnEnter(GameState::Run)))
        // ! Level
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Run)))
        // ! Animation
        .add_system(animate_sprites)
        .add_system(update_direction)
        // ! Movement
        .add_system(move_player_system)
        .insert_resource(LevelSelection::Uid(0))
        .run();
}
