use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}
impl AnimationTimer {
    pub fn new(fps: Option<f32>) -> Self {
        AnimationTimer {
            timer: Timer::from_seconds(1.0 / fps.unwrap_or(12.0), TimerMode::Repeating),
        }
    }
}
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer {
            timer: Timer::from_seconds(1.0 / 4.0, TimerMode::Repeating),
        }
    }
}
#[derive(Component)]
pub struct DespawnWhenAnimationFinished;

#[derive(Eq, PartialEq)]
pub enum SpriteDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct DirectionComponent(pub SpriteDirection);

#[derive(Eq, PartialEq)]
pub enum JumpingState {
    Start,
    Top,
    End,
}

#[derive(PartialEq)]
pub enum AnimationStates {
    Idle,
    Running,
    Jumping(f32, f32),
    Hurt,
    Dead,
    Zoom,
}

#[derive(Component, PartialEq)]
pub struct AnimationState {
    pub state: AnimationStates,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            state: AnimationStates::Idle,
        }
    }
}

#[derive(Component)]
pub struct AnimationSprites {
    pub idle: Handle<TextureAtlas>,
    pub running: Handle<TextureAtlas>,
    pub jumping: Handle<TextureAtlas>,
    pub hurt: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct DeathAnimation(pub Handle<TextureAtlas>);
/*
 #[derive(Resource)]
 struct AnimationClips {
     standing: Handle<AnimationClip>,
     running: Handle<AnimationClip>,
 }
enum AnimationState {
    Standing,
    Running(f32),
}

fn animating_system(
    mut query: &mut Query<(
        &mut TnuaAnimatingState<AnimationState>,
        &TnuaPlatformerAnimatingOutput,
        &mut AnimationPlayer,
    )>,
    animation_clips: Res<AnimationClips>,
) {
    for (mut animating_state, animating_output, mut animation_player) in query.iter_mut() {
        match animating_state.update_by_discriminant({
            let speed = animating_output.running_velocity.length();
            if 0.01 < speed {
                AnimationState::Running(speed)
            } else {
                AnimationState::Standing
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let AnimationState::Running(speed) = state {
                    animation_player.set_speed(*speed);
                }
            }
            TnuaAnimatingStateDirective::Alter {
               //  We don't need the old state here, but it's available for transition
               //  animations.
                old_state: _,
                state,
            } => match state {
                AnimationState::Standing => {
                    animation_player
                        .start(animation_clips.standing.clone_weak())
                        .set_speed(1.0)
                        .repeat();
                }
                AnimationState::Running(speed) => {
                    animation_player
                        .start(animation_clips.standing.clone_weak())
                        .set_speed(*speed)
                        .repeat();
                }
            }
        }
    }
}
*/
