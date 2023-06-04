use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::TnuaPlatformerAnimatingOutput;
use leafwing_input_manager::prelude::*;
// use bevy_rapier2d::prelude::*;

pub fn jump_throught_platforms(
    mut player_query: Query<
        (
            &mut CollisionGroups,
            &ActionState<PlayerAction>,
            &TnuaPlatformerAnimatingOutput,
        ),
        With<Player>,
    >,
) {
    for (mut collision_groups, action_state, tnua_output) in player_query.iter_mut() {
        let pass_throught = || {
            if action_state.pressed(PlayerAction::Crouch) {
                return true;
            }
            if let Some(jumping_velocity) = tnua_output.jumping_velocity {
                if jumping_velocity > 0.0 {
                    return true;
                }
            }
            return false;
        };
        if pass_throught() {
            collision_groups.filters = Group::ALL - Group::GROUP_1
        } else {
            collision_groups.filters = Group::ALL
        }
    }
}
