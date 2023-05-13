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
}
#[derive(Eq, PartialEq)]
pub enum SpriteDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct DirectionComponent(pub SpriteDirection);
