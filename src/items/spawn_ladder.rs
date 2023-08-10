use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

pub fn detect_can_climb(
    mut climber_query: Query<(Entity, &mut MovementControl)>,
    climbable_query: Query<Entity, With<Climbable>>,
    rapier_context: Res<RapierContext>,
) {
    for (climber_entity, mut controls) in climber_query.iter_mut() {
        controls.can_climb = climbable_query.iter().any(|climbable_entity| {
            return rapier_context
                .contact_pair(climber_entity, climbable_entity)
                .is_some();
        });
        if !controls.can_climb {
            controls.is_climbing = false;
        }
    }
}
