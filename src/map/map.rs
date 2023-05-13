use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::MyAssets;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
pub fn map_plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.register_ldtk_int_cell::<WallBundle>(2);
    app.register_ldtk_int_cell::<WallBundle>(3);
}

pub fn spawn_map(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.test_level.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
