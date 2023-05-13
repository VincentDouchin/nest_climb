use bevy::prelude::*;

use crate::{AnimationTimerComponent, DirectionComponent, SpriteDirection};

pub fn animate_sprites(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut AnimationTimerComponent,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut sprite, texture_atlas_handle, mut animation_timer) in query.iter_mut() {
        animation_timer.timer.tick(time.delta());
        if animation_timer.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

pub fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &DirectionComponent)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.0 == SpriteDirection::Left
    }
}
