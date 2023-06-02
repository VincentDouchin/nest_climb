use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationTimerComponent {
    pub timer: Timer,
}

impl AnimationTimerComponent {
    pub fn new(fps: Option<f32>) -> Self {
        AnimationTimerComponent {
            timer: Timer::from_seconds(1.0 / fps.unwrap_or(8.0), TimerMode::Repeating),
        }
    }
    pub fn finished(&self) -> bool {
        true
    }
}
impl Default for AnimationTimerComponent {
    fn default() -> Self {
        AnimationTimerComponent {
            timer: Timer::from_seconds(1.0 / 8.0, TimerMode::Repeating),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum SpriteDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct DirectionComponent(pub SpriteDirection);

#[derive(Eq, PartialEq)]
pub enum AnimationStates {
    Idle,
    Running,
    Jumping,
    Hurt,
    Dead,
    Zoom,
}

#[derive(Component, Eq, PartialEq)]
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
