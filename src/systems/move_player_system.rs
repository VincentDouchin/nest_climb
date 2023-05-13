use bevy::prelude::*;
use bevy_tnua::*;

use crate::{DirectionComponent, SpriteDirection};
// https://www.millisecond.com/support/docs/current/html/language/scancodes.htm
pub fn move_player_system(
    mut player_query: Query<(&mut TnuaPlatformerControls, &mut DirectionComponent)>,
    keys: Res<Input<ScanCode>>,
) {
    for (mut controls, mut direction) in player_query.iter_mut() {
        if keys.pressed(ScanCode(75)) || keys.pressed(ScanCode(30)) {
            controls.desired_forward = -Vec3::X;
            controls.desired_velocity = -Vec3::X;
            direction.0 = SpriteDirection::Left;
        } else if keys.pressed(ScanCode(77)) || keys.pressed(ScanCode(32)) {
            controls.desired_forward = Vec3::X;
            controls.desired_velocity = Vec3::X;
            direction.0 = SpriteDirection::Right;
        } else {
            controls.desired_velocity = Vec3::ZERO
        }
        if keys.just_pressed(ScanCode(57)) {
            controls.jump = Some(1.0);
        } else {
            controls.jump = None;
        }
    }
}
