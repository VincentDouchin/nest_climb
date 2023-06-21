use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Left,
    Right,
    Up,
    Down,
    Pause,
    Select,
}
pub fn get_menu_input_map() -> InputMap<MenuAction> {
    use MenuAction::*;
    let mut input_map = InputMap::default();

    // Up
    input_map.insert(KeyCode::Up, Up);
    input_map.insert(GamepadButtonType::DPadUp, Up);
    // Left
    input_map.insert(KeyCode::Left, Left);
    input_map.insert(GamepadButtonType::DPadLeft, Left);
    // Right
    input_map.insert(KeyCode::Right, Right);
    input_map.insert(GamepadButtonType::DPadRight, Right);
    // Down
    input_map.insert(KeyCode::Down, Down);
    input_map.insert(GamepadButtonType::DPadDown, Down);
    // Pause
    input_map.insert(KeyCode::Escape, Pause);
    input_map.insert(GamepadButtonType::Start, Pause);
    // Select
    input_map.insert(KeyCode::Return, Select);
    input_map.insert(GamepadButtonType::East, Select);
    return input_map;
}

pub fn spawn_inputs(mut commands: Commands) {
    dbg!(get_menu_input_map());
    commands.spawn(InputManagerBundle::<MenuAction> {
        input_map: get_menu_input_map(),
        ..default()
    });
}
