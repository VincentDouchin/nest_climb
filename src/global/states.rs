use bevy::{prelude::*, window::WindowFocused};
use bevy_rapier2d::prelude::*;

use crate::{spawn_pause_ui, PauseUi};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Run,
    Start,
    LevelSelect,
}
#[derive(Resource)]
pub struct Paused {
    enabled: bool,
}
impl Default for Paused {
    fn default() -> Self {
        Paused { enabled: false }
    }
}

pub fn pause_game(
    keys: Res<Input<KeyCode>>,
    mut paused: ResMut<Paused>,
    mut events: EventReader<WindowFocused>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut time: ResMut<Time>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        paused.enabled = !paused.enabled
    }
    for event in events.iter() {
        if !event.focused {
            paused.enabled = true
        }
    }
    rapier_config.physics_pipeline_active = !paused.enabled;
    if paused.enabled {
        time.pause()
    } else {
        time.unpause()
    }
}

pub fn pause_transition_true(paused: Res<Paused>) -> bool {
    return paused.is_changed() && paused.enabled == true;
}
pub fn pause_transition_false(paused: Res<Paused>) -> bool {
    return paused.is_changed() && paused.enabled == false;
}
pub fn despawn_pause_ui(mut commands: Commands, query: Query<Entity, With<PauseUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
pub fn pause_plugin(app: &mut App) {
    app.init_resource::<Paused>()
        .add_system(pause_game)
        .add_system(spawn_pause_ui.run_if(pause_transition_true))
        .add_system(despawn_pause_ui.run_if(pause_transition_false));
}
