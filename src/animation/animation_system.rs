use crate::*;
use bevy::prelude::*;
use bevy_tnua::*;

pub fn update_animation_state(
    mut query: Query<(
        &TnuaPlatformerAnimatingOutput,
        &mut AnimationState,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
        Option<&Health>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (state, mut animation_state, sprite, texture_atlas_handle, maybe_health) in query.iter_mut()
    {
        let texture_atlas = texture_atlases
            .get(texture_atlas_handle)
            .expect("texture atlas exists");
        let finished = sprite.index == texture_atlas.len() - 1;

        let lost_health = if let Some(health) = maybe_health {
            health.took_damage
        } else {
            false
        };

        let state = if lost_health {
            AnimationStates::Hurt
        } else if state.jumping_velocity.is_some() {
            AnimationStates::Jumping
        } else if state.running_velocity.x.abs() > 0.0 {
            AnimationStates::Running
        } else {
            AnimationStates::Idle
        };
        if !(animation_state.state == AnimationStates::Hurt && !finished) {
            animation_state.state = state
        }
    }
}

pub fn change_animation_atlas(
    mut query: Query<(
        &AnimationState,
        &AnimationSprites,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animation_state, sprites, mut texture_atlas_handle, mut sprite) in query.iter_mut() {
        let last_atlas = texture_atlas_handle.clone();
        *texture_atlas_handle = match animation_state.state {
            AnimationStates::Jumping => sprites.jumping.clone(),
            AnimationStates::Running => sprites.running.clone(),
            AnimationStates::Hurt => sprites.hurt.clone(),
            _ => sprites.idle.clone(),
        };
        if last_atlas != texture_atlas_handle.clone() {
            sprite.index = 0
        }
    }
}

pub fn animate_sprites(
    mut query: Query<(
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut AnimationTimerComponent,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas_handle, mut sprite, mut animation_timer) in query.iter_mut() {
        animation_timer.timer.tick(time.delta());
        if animation_timer.timer.just_finished() {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

pub fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &DirectionComponent)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.0 == SpriteDirection::Left
    }
}