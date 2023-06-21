use bevy::prelude::{GamepadButtonType, KeyCode, ScanCode};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Run,
    Jump,
    Crouch,
    Pause,
    Select,
}
pub fn get_player_input_map() -> InputMap<PlayerAction> {
    use PlayerAction::*;
    let mut input_map = InputMap::default();

    // Jump
    input_map.insert(KeyCode::Space, Jump);
    input_map.insert(GamepadButtonType::East, Jump);
    // Move Left
    input_map.insert(KeyCode::Left, MoveLeft);
    input_map.insert(ScanCode(81), MoveLeft);
    input_map.insert(GamepadButtonType::DPadLeft, MoveLeft);
    // Move Right
    input_map.insert(KeyCode::Right, MoveRight);
    input_map.insert(ScanCode(68), MoveRight);
    input_map.insert(GamepadButtonType::DPadRight, MoveRight);
    // Crouch
    input_map.insert(KeyCode::Down, Crouch);
    input_map.insert(GamepadButtonType::DPadDown, Crouch);
    // Pause
    input_map.insert(KeyCode::Escape, Pause);
    input_map.insert(GamepadButtonType::Start, Pause);
    // Select
    input_map.insert(KeyCode::Return, Select);
    input_map.insert(GamepadButtonType::East, Select);

    return input_map;
}
