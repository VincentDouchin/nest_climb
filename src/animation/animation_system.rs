use crate::*;
use bevy::prelude::*;

pub fn update_animation_state(
    mut query: Query<(
        &Velocity,
        &MovementControl,
        &mut AnimationState,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut SpriteDirection,
        Option<&Health>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (
        velocity,
        controls,
        mut animation_state,
        sprite,
        texture_atlas_handle,
        mut direction,
        maybe_health,
    ) in query.iter_mut()
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
        } else if velocity.linvel.y > 1.0 && !controls.grounded {
            AnimationStates::JumpingUp
        } else if velocity.linvel.y < -1.0 && !controls.grounded {
            AnimationStates::JumpingDown
        } else if velocity.linvel.x.abs() > 1.0 {
            AnimationStates::Running
        } else {
            AnimationStates::Idle
        };
        if velocity.linvel.x > 1.0 {
            *direction = SpriteDirection::Right
        } else if velocity.linvel.x < -1.0 {
            *direction = SpriteDirection::Left
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
            AnimationStates::JumpingUp => sprites.jumping_up.clone(),
            AnimationStates::JumpingDown => sprites.jumping_down.clone(),
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
        &mut AnimationTimer,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas_handle, mut sprite, mut animation_timer) in query.iter_mut() {
        if animation_timer.state != AnimationTimerState::Stopped {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            animation_timer.timer.tick(time.delta());

            if animation_timer.state == AnimationTimerState::Once
                && sprite.index >= texture_atlas.len() - 1
            {
                animation_timer.state = AnimationTimerState::Stopped;
                sprite.index = 0;
                continue;
            }
            if animation_timer.timer.just_finished() {
                sprite.index = (sprite.index + 1) % texture_atlas.len();
            }
        }
    }
}

pub fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &SpriteDirection)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction == &SpriteDirection::Left
    }
}

pub fn despawn_entity_when_animation_finished(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &TextureAtlasSprite,
            &Handle<TextureAtlas>,
            Option<&Ghost>,
        ),
        With<DespawnWhenAnimationFinished>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    for (entity, sprite, atlas_handle, maybe_ghost) in query.iter() {
        if let Some(atlas) = texture_atlases.get(&atlas_handle) {
            if sprite.index == atlas.len() - 1 {
                commands.entity(entity).despawn_recursive();
                if maybe_ghost.is_some() {
                    next_state.set(PauseState::GameOver)
                }
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
