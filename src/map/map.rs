use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::*;
#[derive(Component)]
pub struct Level;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub fn map_plugin(app: &mut App) {
    app.insert_resource(LevelSelection::Index(0));
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.register_ldtk_int_cell::<WallBundle>(2);
    app.register_ldtk_int_cell::<WallBundle>(3);
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.register_ldtk_entity::<EnemyBundle>("Enemy");
    app.register_ldtk_entity::<CollectibleBundle>("Collectible");
    app.insert_resource(LdtkSettings {
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    });
    app.add_system(spawn_map.in_schedule(OnExit(GameState::LevelSelect)));
    app.add_system(despawn_map.in_schedule(OnEnter(GameState::LevelSelect)));
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub level: Handle<LdtkAsset>,
}

pub fn spawn_map(mut commands: Commands, current_level: Res<CurrentLevel>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: current_level.level.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Level,
    ));
}
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Level>>) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
