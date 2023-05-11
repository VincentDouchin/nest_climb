use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl AnimationTimer {
    pub fn new(fps: Option<f32>) -> Self {
        AnimationTimer {
            timer: Timer::from_seconds(1.0 / fps.unwrap_or(8.0), TimerMode::Repeating),
        }
    }
}
