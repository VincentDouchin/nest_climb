use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Level;

pub fn map_plugin(app: &mut App) {
    app.insert_resource(LevelSelection::Index(0))
        .init_resource::<CurrentLevel>()
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_int_cell::<WallBundle>(4)
        .register_ldtk_int_cell::<WallBundle>(5)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_entity::<CollectibleBundle>("Collectible")
        .register_ldtk_entity::<FlagBundle>("Flag")
        .register_ldtk_entity::<PlatformBundle>("Platform")
        .register_ldtk_entity::<SawbladeBundle>("Sawblade")
        .register_ldtk_entity::<PendulumBundle>("Pendulum")
        .register_ldtk_entity::<HeartBundle>("Heart")
        .register_ldtk_entity::<TrampolineBundle>("Trampoline")
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::No,
            ..Default::default()
        })
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Run)))
        .add_system(despawn_map.in_schedule(OnExit(GameState::Run)))
        .add_systems(
            (
                spawn_player,
                spawn_walls,
                spawn_enemy,
                spawn_collectibles,
                spawn_platforms,
                spawn_sawblade,
                spawn_pendulum,
                spawn_flag,
                spawn_heart,
                spawn_trampoline,
            )
                .in_set(OnUpdate(GameState::Run)),
        );
}

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub file: Option<Handle<LdtkAsset>>,
}

pub fn spawn_map(mut commands: Commands, maybe_current_level: Res<CurrentLevel>) {
    if let Some(current_level) = &maybe_current_level.file {
        commands.spawn((
            LdtkWorldBundle {
                ldtk_handle: current_level.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Level,
        ));
    }
}
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Level>>) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
