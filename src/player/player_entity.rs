use crate::{
    get_player_input_map, AnimationTimerComponent, CameraTarget, DirectionComponent, Health,
    MyAssets, PlayerAction, SpriteDirection,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Clone)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    player_query: Query<Entity, Added<EntityInstance>>,
    assets: Res<MyAssets>,
) {
    for entity in player_query.iter() {
        let bundle = (
            assets.bird.clone(),
            TextureAtlasSprite::default(),
            AnimationTimerComponent::new(None),
            DirectionComponent(SpriteDirection::Right),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(10.0, 12.0),
            TnuaRapier2dSensorShape(Collider::cuboid(10.0, 12.0)),
            Velocity::default(),
            Player,
            CameraTarget,
            TnuaPlatformerAnimatingOutput::default(),
            TnuaPlatformerBundle {
                config: TnuaPlatformerConfig {
                    full_speed: 100.0,
                    full_jump_height: 100.0,
                    up: Vec3::Y,
                    forward: Vec3::X,
                    float_height: 0.1,
                    cling_distance: 1.0,
                    spring_strengh: 40.0,
                    spring_dampening: 0.4,
                    acceleration: 200.0,
                    air_acceleration: 100.0,
                    coyote_time: 0.15,
                    jump_input_buffer_time: 0.2,
                    held_jump_cooldown: None,
                    jump_start_extra_gravity: 30.0,
                    jump_fall_extra_gravity: 20.0,
                    jump_shorten_extra_gravity: 40.0,
                    jump_peak_prevention_at_upward_velocity: 0.0,
                    jump_peak_prevention_extra_gravity: 20.0,
                    free_fall_behavior: TnuaFreeFallBehavior::LikeJumpShorten,
                    tilt_offset_angvel: 5.0,
                    tilt_offset_angacl: 500.0,
                    turning_angvel: 10.0,
                },
                ..default()
            },
            InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: get_player_input_map(),
            },
            Health::new(5),
        );
        commands.entity(entity).insert(bundle);
    }
}
