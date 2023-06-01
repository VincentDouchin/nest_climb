use crate::*;
use bevy::prelude::*;
use bevy_tnua::*;

pub fn animate_sprites(
    mut query: Query<(
        &TnuaPlatformerAnimatingOutput,
        &AnimationSprites,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut AnimationTimerComponent,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (state, sprites, mut texture_atlas_handle, mut sprite, mut animation_timer) in
        query.iter_mut()
    {
        let last_atlas = texture_atlas_handle.clone();
        if state.jumping_velocity.is_some() {
            *texture_atlas_handle = sprites.jumping.clone()
        } else if state.running_velocity.x.abs() > 0.0 {
            *texture_atlas_handle = sprites.running.clone()
        } else {
            *texture_atlas_handle = sprites.idle.clone()
        }
        if last_atlas != texture_atlas_handle.clone() {
            sprite.index = 0
        }
        animation_timer.timer.tick(time.delta());
        if animation_timer.timer.just_finished() {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

// pub fn update_sprites(query: Query<(&Handle<TextureAtlas>, &AnimationsSprites, Tnua)>) {}

pub fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &DirectionComponent)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.0 == SpriteDirection::Left
    }
}

#[derive(Component)]
pub struct AnimationSprites {
    pub idle: Handle<TextureAtlas>,
    pub running: Handle<TextureAtlas>,
    pub jumping: Handle<TextureAtlas>,
}
