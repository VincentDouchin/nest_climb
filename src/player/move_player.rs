use bevy::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;

use crate::{DirectionComponent, PlayerAction, SpriteDirection};

pub fn move_player_system(
    mut player_query: Query<(
        &mut TnuaPlatformerControls,
        &mut DirectionComponent,
        &ActionState<PlayerAction>,
    )>,
) {
    for (mut controls, mut direction, actions) in player_query.iter_mut() {
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
        if actions.just_pressed(PlayerAction::Jump) {
            controls.jump = Some(1.0);
        } else {
            controls.jump = None;
        }
    }
}
