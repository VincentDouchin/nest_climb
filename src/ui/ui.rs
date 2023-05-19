use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct StateUi {
    pub state: GameState,
}

pub fn despawn_state_ui(
    mut commands: Commands,
    ui_query: Query<(Entity, &StateUi)>,
    app_state: Res<State<GameState>>,
    mut last_state: Local<GameState>,
) {
    if app_state.is_changed() {
        for (ui_entity, state_ui) in ui_query.iter() {
            if state_ui.state == last_state.clone() {
                commands.entity(ui_entity).despawn_recursive()
            }
        }
        *last_state = app_state.0.clone();
    }
}
