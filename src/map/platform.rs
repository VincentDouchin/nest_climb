use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::TnuaPlatformerAnimatingOutput;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct GhostPlatform;

pub fn spawn_ghost_platforms(
    platform_query: Query<(Entity, &Collider), (With<GhostPlatform>, Without<CollisionGroups>)>,
    mut commands: Commands,
) {
    for (entity, collider) in platform_query.iter() {
        commands
            .entity(entity)
            .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL))
            .with_children(|platform| {
                platform.spawn((
                    TransformBundle::default(),
                    collider.clone(),
                    RigidBody::Fixed,
                    Wall::Platform,
                    Sensor,
                ));
            });
    }
}

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
