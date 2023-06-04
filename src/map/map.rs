use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
#[derive(Component)]
pub struct Level;

pub fn map_plugin(app: &mut App) {
    app.insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_int_cell::<WallBundle>(4)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_entity::<CollectibleBundle>("Collectible")
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_system(spawn_map.in_schedule(OnExit(GameState::LevelSelect)))
        .add_system(despawn_map.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(jump_throught_platforms.in_set(OnUpdate(GameState::Run)));
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
