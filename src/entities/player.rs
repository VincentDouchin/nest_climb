use crate::{AnimationTimerComponent, CameraTarget, DirectionComponent, MyAssets, SpriteDirection};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;

#[derive(Component)]
pub struct PlayerController {
    pub grounded: bool,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController { grounded: true }
    }
}

pub fn spawn_player(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            texture_atlas: assets.bird.clone(),
            ..default()
        },
        AnimationTimerComponent::new(None),
        DirectionComponent(SpriteDirection::Right),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(10.0, 12.0),
        // Collider::capsule_y(10.0, 1.0),
        Velocity::default(),
        PlayerController::default(),
        CameraTarget,
        TnuaPlatformerAnimatingOutput::default(),
        TnuaPlatformerBundle {
            config: TnuaPlatformerConfig {
                full_speed: 100.0,
                full_jump_height: 80.0,
                up: Vec3::Y,
                forward: Vec3::X,
                float_height: 12.0,
                cling_distance: 1.0,
                spring_strengh: 100.0,
                spring_dampening: 0.4,
                acceleration: 200.0,
                air_acceleration: 20.0,
                coyote_time: 0.15,
                jump_input_buffer_time: 0.2,
                held_jump_cooldown: None,
                jump_start_extra_gravity: 30.0,
                jump_fall_extra_gravity: 60.0,
                jump_shorten_extra_gravity: 40.0,
                jump_peak_prevention_at_upward_velocity: 0.0,
                jump_peak_prevention_extra_gravity: 50.0,
                free_fall_behavior: TnuaFreeFallBehavior::LikeJumpShorten,
                tilt_offset_angvel: 0.0,
                tilt_offset_angacl: 0.0,
                turning_angvel: 10.0,
            },
            ..default()
        },
    ));
}
