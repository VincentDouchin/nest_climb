use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{DirectionComponent, PlayerController, SpriteDirection};
// https://www.millisecond.com/support/docs/current/html/language/scancodes.htm
pub fn move_player_system(
    mut player_query: Query<(&mut Velocity, &mut DirectionComponent), With<PlayerController>>,
    time: Res<Time>,
    keys: Res<Input<ScanCode>>,
) {
    for (mut velocity, mut direction) in player_query.iter_mut() {
        if keys.pressed(ScanCode(75)) || keys.pressed(ScanCode(30)) {
            velocity.linvel.x -= 100.0 * time.delta_seconds();
            direction.0 = SpriteDirection::Left
        }
        if keys.pressed(ScanCode(77)) || keys.pressed(ScanCode(32)) {
            velocity.linvel.x += 100.0 * time.delta_seconds();
            direction.0 = SpriteDirection::Right
        }
        if keys.just_pressed(ScanCode(57)) {
            velocity.linvel.y += 10000.0 * time.delta_seconds();
        }
    }
}

pub fn ground_player(player_query:Query<>)