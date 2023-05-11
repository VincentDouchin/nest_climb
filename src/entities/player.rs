use crate::{AnimationTimer, MyAssets};
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, assets: Res<MyAssets>) {
    println!("ok");
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            texture_atlas: assets.bird.clone(),
            ..default()
        },
        AnimationTimer::new(None),
    ));
}
