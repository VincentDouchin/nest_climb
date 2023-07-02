use bevy::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;

use crate::{BouncingOnTrampoline, DirectionComponent, PlayerAction, SpriteDirection};

pub fn move_player_system(
    mut player_query: Query<(
        &mut TnuaPlatformerControls,
        &mut DirectionComponent,
        &ActionState<PlayerAction>,
        &BouncingOnTrampoline,
    )>,
) {
    for (mut controls, mut direction, actions, bouncing) in player_query.iter_mut() {
        // ! Movement
        if actions.pressed(PlayerAction::MoveLeft) {
            controls.desired_forward = Vec3::NEG_X;
            controls.desired_velocity = Vec3::NEG_X;
            direction.0 = SpriteDirection::Left;
        } else if actions.pressed(PlayerAction::MoveRight) {
            controls.desired_forward = Vec3::X;
            controls.desired_velocity = Vec3::X;
            direction.0 = SpriteDirection::Right;
        } else {
            controls.desired_velocity = Vec3::ZERO
        }

        // ! Jump
        if bouncing.0.is_none() {
            controls.jump = if actions.pressed(PlayerAction::Jump) {
                Some(1.0)
            } else {
                None
            };
        }
    }
}
