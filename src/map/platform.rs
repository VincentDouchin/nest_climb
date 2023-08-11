use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::TnuaPlatformerAnimatingOutput;
use leafwing_input_manager::prelude::*;

// #[derive(Component, Default)]
// pub struct GhostPlatform;

#[derive(Component, Default)]
pub struct DisappearingPlatform(pub Option<Timer>);

// pub fn spawn_ghost_platforms(
//     platform_query: Query<(Entity, &Collider), (With<GhostPlatform>, Without<Sensor>)>,
//     mut commands: Commands,
// ) {
// for (entity, collider) in platform_query.iter() {
//     commands
//         .entity(entity)
//         .insert(Sensor)
//         .with_children(|platform| {
//             platform.spawn((
//                 TransformBundle::default(),
//                 collider.clone(),
//                 RigidBody::Fixed,
//                 Wall::Platform,
//                 CollisionGroups::new(Group::GROUP_1, Group::ALL),
//             ));
//         });
// }
// }

pub fn jump_throught_platforms(
    mut player_query: Query<
        (
            Entity,
            &mut CollisionGroups,
            &ActionState<PlayerAction>,
            &TnuaPlatformerAnimatingOutput,
        ),
        With<Player>,
    >,
    platform_sensor_query: Query<(Entity, &Wall), With<Sensor>>,
    rapier_context: Res<RapierContext>,
) {
    for (player_entity, mut collision_groups, action_state, tnua_output) in player_query.iter_mut()
    {
        let collide_with_platform_sensor = || {
            for (platform_entity, wall) in platform_sensor_query.iter() {
                if let Some(contact_pair) =
                    rapier_context.intersection_pair(player_entity, platform_entity)
                {
                    return contact_pair && wall == &Wall::Platform;
                }
            }
            return false;
        };
        if action_state.pressed(PlayerAction::Crouch)
            || tnua_output
                .jumping_velocity
                .map_or(false, |jumping_velocity| jumping_velocity > 0.0)
        {
            collision_groups.filters = Group::ALL - Group::GROUP_1;
        } else if !collide_with_platform_sensor() {
            collision_groups.filters = Group::ALL
        }
    }
}

pub fn disappear_platforms(
    mut query: Query<(Entity, &mut DisappearingPlatform)>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        for (entity, mut disappearing_platform) in query.iter_mut() {
            if disappearing_platform.0.is_none() {
                if rapier_context.contact_pair(entity, player_entity).is_some() {
                    disappearing_platform.0 = Some(Timer::from_seconds(3.0, TimerMode::Once));
                }
            }
            if let Some(timer) = disappearing_platform.0.as_mut() {
                timer.tick(time.delta());
                if timer.finished() {
                    commands.entity(entity).despawn_recursive()
                }
            }
        }
    }
}
