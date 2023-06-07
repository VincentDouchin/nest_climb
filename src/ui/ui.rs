use bevy::prelude::*;

#[derive(Component)]
pub struct StateUi<T: States>(pub T);

pub fn despawn_state_ui<T: States>(
    mut commands: Commands,
    ui_query: Query<(Entity, &StateUi<T>)>,
    app_state: Res<State<T>>,
    mut last_state: Local<T>,
) {
    if app_state.is_changed() {
        for (ui_entity, state_ui) in ui_query.iter() {
            if state_ui.0 == last_state.clone() {
                commands.entity(ui_entity).despawn_recursive()
            }
        }
        *last_state = app_state.0.clone();
    }
}
