use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub fn map_plugin(app: &mut App) {
    app.add_system(spawn_map.in_schedule(OnEnter(GameState::InitRun)));
    app.insert_resource(LevelSelection::Index(0));
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.register_ldtk_int_cell::<WallBundle>(2);
    app.register_ldtk_int_cell::<WallBundle>(3);
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.register_ldtk_entity::<EnemyBundle>("Enemy");
    app.insert_resource(LdtkSettings {
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    });
}

pub fn spawn_map(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.test_level.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    next_state.set(GameState::Run);
}
