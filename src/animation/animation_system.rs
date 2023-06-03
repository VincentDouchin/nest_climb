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
    for (animation_output, mut animation_state, sprite, texture_atlas_handle, maybe_health) in
        query.iter_mut()
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
        } else if let Some(jumping_velocity) = animation_output.jumping_velocity {
            if let AnimationStates::Jumping(max, _) = animation_state.state {
                AnimationStates::Jumping(max, jumping_velocity)
            } else {
                AnimationStates::Jumping(jumping_velocity, jumping_velocity)
            }
        } else if animation_output.running_velocity.x.abs() > 0.0 {
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
            AnimationStates::Jumping(_, _) => sprites.jumping.clone(),
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
        Option<&AnimationState>,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas_handle, mut sprite, mut animation_timer, maybe_animation_state) in
        query.iter_mut()
    {
        let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
        animation_timer.timer.tick(time.delta());
        let jumping_velocity = || {
            if let Some(animation_state) = maybe_animation_state {
                if let AnimationStates::Jumping(max_velocity, current_velocity) =
                    animation_state.state
                {
                    return Some((max_velocity, current_velocity));
                }
            };
            return None;
        };
        fn get_index(current: f32, max: f32, len: usize) -> usize {
            let length = len as f32 - 1.0;
            return ((length - (current + max) / (max * 2.0) * length).floor()).min(length)
                as usize;
        }

        if let Some((max_velocity, current_velocity)) = jumping_velocity() {
            sprite.index = get_index(current_velocity, max_velocity, texture_atlas.len());
        } else if animation_timer.timer.just_finished() {
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

pub fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &DirectionComponent)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.0 == SpriteDirection::Left
    }
}

pub fn despawn_entity_when_animation_finished(
    mut commands: Commands,
    query: Query<
        (Entity, &TextureAtlasSprite, &Handle<TextureAtlas>),
        With<DespawnWhenAnimationFinished>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (entity, sprite, atlas_handle) in query.iter() {
        if let Some(atlas) = texture_atlases.get(&atlas_handle) {
            if sprite.index == atlas.len() - 1 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn animation_plugin(app: &mut App) {
    app.add_systems(
        (change_animation_atlas, animate_sprites)
            .chain()
            .in_set(OnUpdate(GameState::Run)),
    )
    .add_systems(
        (
            update_direction,
            despawn_entity_when_animation_finished,
            update_animation_state,
        )
            .in_set(OnUpdate(GameState::Run)),
    );
}
