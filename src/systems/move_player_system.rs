use bevy::prelude::*;

use crate::{PlayerController, PlayerInputs};

pub fn move_player_system(
    mut player_query: Query<&mut Transform, With<PlayerController>>,
    player_inputs: Res<PlayerInputs>,
    time: Res<Time>,
) {
    for mut transform in player_query.iter_mut() {
        if player_inputs.right {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if player_inputs.left {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}
