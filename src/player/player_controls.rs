use bevy::prelude::{GamepadButtonType, KeyCode, ScanCode};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Run,
    Jump,
}
pub fn get_player_input_map() -> InputMap<PlayerAction> {
    use PlayerAction::*;
    let mut input_map = InputMap::default();

    // Movement
    input_map.insert(KeyCode::Space, Jump);
    input_map.insert(GamepadButtonType::East, Jump);

    input_map.insert(KeyCode::Left, MoveLeft);
    input_map.insert(ScanCode(81), MoveLeft);
    input_map.insert(GamepadButtonType::DPadLeft, MoveLeft);

    input_map.insert(KeyCode::Right, MoveRight);
    input_map.insert(ScanCode(68), MoveRight);
    input_map.insert(GamepadButtonType::DPadRight, MoveRight);

    return input_map;
}
