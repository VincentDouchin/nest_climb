use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Default, Clone)]
pub struct GhostPlatform(pub bool);

#[derive(Component, Default)]
pub struct DisappearingPlatform(pub Option<Timer>);

pub fn spawn_ghost_platforms(
    platform_query: Query<Entity, Added<GhostPlatform>>,
    mut commands: Commands,
) {
    for entity in platform_query.iter() {
        commands.entity(entity).insert(SolverGroups {
            memberships: Group::ALL,
            filters: Group::ALL,
        });
    }
}
pub fn jump_through_platforms(
    player_query: Query<(Entity, &ActionState<PlayerAction>), With<Player>>,
    mut platform_query: Query<(Entity, &mut SolverGroups, &mut GhostPlatform)>,
    rapier_context: Res<RapierContext>,
) {
    for (player_entity, actions) in player_query.iter() {
        for (platform_entity, mut groups, mut platform) in platform_query.iter_mut() {
            let contact = rapier_context.contact_pair(player_entity, platform_entity);
            let is_contact = contact.is_some();
            let coming_from_the_top = contact.map_or(false, |contact_pair_view| {
                let is_player_first = contact_pair_view.collider1() == player_entity;
                contact_pair_view
                    .find_deepest_contact()
                    .map_or(true, |(deepest_contact, _view)| {
                        (deepest_contact.normal().y < 0.0) == is_player_first
                    })
            });
            let is_crouching = actions.pressed(PlayerAction::Crouch);
            let should_collide = is_contact && !is_crouching && coming_from_the_top;
            if !should_collide && is_contact {
                platform.0 = true
            }
            if !is_contact {
                platform.0 = false
            }

            if should_collide && !platform.0 {
                groups.filters = Group::ALL
            } else {
                groups.filters = Group::NONE
            };
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
