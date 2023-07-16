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
        let image_size = Vec2::splat(32.0);
        let player_size = Vec2::new(14.0 / 2.0, 24.0 / 2.0);
        let collider = Collider::cuboid(player_size.x, player_size.y);
        let tnua_bundle = (
            TnuaPlatformerAnimatingOutput::default(),
            TnuaRapier2dIOBundle::default(),
            TnuaRapier2dSensorShape(collider.clone()),
            TnuaPlatformerBundle {
                config: TnuaPlatformerConfig {
                    full_speed: 100.0,
                    full_jump_height: 50.0,
                    up: Vec3::Y,
                    forward: Vec3::X,
                    float_height: 1.0,
                    cling_distance: 1.0,
                    spring_strengh: 500.0,
                    spring_dampening: 0.4,
                    acceleration: 200.0,
                    air_acceleration: 100.0,
                    coyote_time: 0.15,
                    jump_input_buffer_time: 0.2,
                    held_jump_cooldown: None,
                    jump_fall_extra_gravity: 150.0,
                    jump_shorten_extra_gravity: 600.0,
                    jump_peak_prevention_at_upward_velocity: 0.0,
                    jump_peak_prevention_extra_gravity: 20.0,
                    free_fall_behavior: TnuaFreeFallBehavior::LikeJumpShorten,
                    tilt_offset_angvel: 0.0,
                    tilt_offset_angacl: 0.0,
                    turning_angvel: 10.0,
                    upslope_jump_extra_gravity: 0.0,
                    jump_takeoff_extra_gravity: 0.0,
                    jump_takeoff_above_velocity: 0.0,
                    height_change_impulse_for_duration: 0.04,
                    height_change_impulse_limit: 0.0,
                },
                ..default()
            },
        );
        let animation_bundle = (
            DirectionComponent(SpriteDirection::Right),
            AnimationTimer::new(8.0),
            AnimationState::default(),
            DeathAnimation(assets.bird_death.clone()),
            assets.bird_idle.clone(),
            AnimationSprites {
                idle: assets.bird_idle.clone(),
                running: assets.bird_run.clone(),
                jumping_up: assets.bird_jump_up.clone(),
                jumping_down: assets.bird_jump_down.clone(),
                hurt: assets.bird_hurt.clone(),
            },
            TextureAtlasSprite {
                anchor: Anchor::Custom(Vec2::new(
                    0.0,
                    ((-player_size.y / 2.0) / image_size.y) / 2.0,
                )),
                ..default()
            },
        );
        let physics_bundle = (
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            collider,
            Velocity::default(),
            CollisionGroups::new(Group::GROUP_2, Group::ALL),
            Sleeping::disabled(),
            Ccd::default(),
        );
        let player_bundle = (
            CameraTarget::new(false, true),
            InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: get_player_input_map(),
            },
            Health::new(5),
            BouncingOnTrampoline::default(),
        );
        commands
            .entity(entity)
            .insert(tnua_bundle)
            .insert(player_bundle)
            .insert(animation_bundle)
            .insert(physics_bundle);
    }
}

pub fn is_player_alive(query: Query<&Player>) -> bool {
    return query.iter().len() > 1;
}
