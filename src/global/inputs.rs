use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Left,
    Right,
    Up,
    Down,
    Pause,
    UnPause,
    Select,
}
pub fn get_menu_input_map() -> InputMap<MenuAction> {
    use MenuAction::*;
    let mut input_map = InputMap::default();

    // Pause
    input_map.insert(KeyCode::Escape, Pause);
    input_map.insert(GamepadButtonType::Start, Pause);
    input_map.insert(KeyCode::Escape, UnPause);
    input_map.insert(GamepadButtonType::Start, UnPause);

    return input_map;
}

pub fn spawn_inputs(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<MenuAction> {
        input_map: get_menu_input_map(),
        ..default()
    });
}
