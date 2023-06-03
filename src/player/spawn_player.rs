use crate::*;
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
}

pub fn spawn_player(
    mut commands: Commands,
    player_query: Query<Entity, Added<Player>>,
    assets: Res<MyAssets>,
) {
    for entity in player_query.iter() {
        let collider = Collider::cuboid(7.0, 10.5);
        let bundle = (
            assets.bird_idle.clone(),
            AnimationSprites {
                idle: assets.bird_idle.clone(),
                running: assets.bird_run.clone(),
                jumping: assets.bird_jump.clone(),
                hurt: assets.bird_hurt.clone(),
            },
            TextureAtlasSprite {
                anchor: Anchor::Custom(Vec2::new(0.0, (-10.5 / 32.0) / 2.0)),
                ..default()
            },
            AnimationTimer::default(),
            DirectionComponent(SpriteDirection::Right),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            TnuaRapier2dSensorShape(collider.clone()),
            collider,
            Velocity::default(),
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
                    jump_fall_extra_gravity: 150.0,
                    jump_shorten_extra_gravity: 600.0,
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
        commands.entity(entity).insert(bundle).insert((
            AnimationState::default(),
            DeathAnimation(assets.bird_death.clone()),
        ));
    }
}
