use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn toggle_debug(mut debug_config: ResMut<DebugRenderContext>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::F1) {
        debug_config.enabled = !debug_config.enabled
    }
}

pub fn debug_plugin(app: &mut App) {
    app.add_system(toggle_debug);
}
