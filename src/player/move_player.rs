use crate::*;
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
        &mut Climber,
        &mut Velocity,
    )>,
) {
    for (mut controls, mut direction, actions, bouncing, mut climber, mut velocity) in
        player_query.iter_mut()
    {
        if climber.is_climbing {
            if actions.pressed(PlayerAction::MoveUp) {
                velocity.linvel.y += 1.0;
            }
            if actions.pressed(PlayerAction::Jump) {
                climber.is_climbing = false;
            }
        } else {
            if climber.can_climb
                && (actions.pressed(PlayerAction::MoveUp)
                    || actions.pressed(PlayerAction::MoveDown))
            {
                climber.is_climbing = true;
            }
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
}
