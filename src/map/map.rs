use bevy::prelude::*;
use bevy_ecs_ldtk::*;

use crate::MyAssets;

pub fn spawn_map(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.test_level.clone(),
        ..Default::default()
    });
}
